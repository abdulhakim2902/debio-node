use crate::*;
use traits_genetic_analysis_orders::{
	GeneticAnalysisOrderEventEmitter, GeneticAnalysisOrderStatusUpdater,
};

impl<T: Config> GeneticAnalysisOrderInterface<T> for Pallet<T> {
	type GeneticAnalysisOrder = GeneticAnalysisOrderOf<T>;
	type Error = Error<T>;

	fn create_genetic_analysis_order(
		customer_id: &T::AccountId,
		genetic_data_id: &T::Hash,
		genetic_analyst_service_id: &T::Hash,
		price_index: u32,
		customer_box_public_key: &T::Hash,
		genetic_link: &[u8],
		asset_id: Option<u32>,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		let genetic_analyst_service =
			T::GeneticAnalystServices::genetic_analyst_service_by_id(genetic_analyst_service_id)
				.ok_or(Error::<T>::GeneticAnalystServiceDoesNotExist)?;

		let seller_id = genetic_analyst_service.get_owner_id();
		if !T::GeneticAnalysts::is_genetic_analyst_available(seller_id) {
			// If _bool is false, then genetic analyst is unavailable
			return Err(Error::<T>::GeneticAnalystUnavailable)
		}

		let genetic_data = T::GeneticData::genetic_data_by_id(genetic_data_id)
			.ok_or(Error::<T>::GeneticDataDoesNotExist)?;

		if customer_id != genetic_data.get_owner_id() {
			return Err(Error::<T>::NotOwnerOfGeneticData)
		}

		let prices_by_currency = genetic_analyst_service.get_prices_by_currency();
		if prices_by_currency.is_empty() ||
			prices_by_currency.len() - 1 < price_index.try_into().unwrap()
		{
			return Err(Error::<T>::PriceIndexNotFound)
		}

		let price_by_currency = &prices_by_currency[price_index as usize];

		let total_price = &price_by_currency.total_price;
		let currency = &price_by_currency.currency;
		let asset_id = Self::do_validate_asset_id(currency, asset_id)?;
		let prices = &price_by_currency.price_components;
		let additional_prices = &price_by_currency.additional_prices;

		let now = pallet_timestamp::Pallet::<T>::get();

		// Initialize GeneticAnalysis
		let genetic_analysis_order_id =
			Self::generate_genetic_analysis_order_id(customer_id, genetic_analyst_service_id);

		let genetic_analysis = T::GeneticAnalysis::register_genetic_analysis(
			seller_id,
			customer_id,
			&genetic_analysis_order_id,
		)
		.map_err(|_| Error::<T>::GeneticAnalysisInitalizationError)?;

		let genetic_analysis_order = GeneticAnalysisOrder::new(
			genetic_analysis_order_id,
			*genetic_analyst_service_id,
			customer_id.clone(),
			*customer_box_public_key,
			seller_id.clone(),
			*genetic_data_id,
			genetic_analysis.get_genetic_analysis_tracking_id().clone(),
			genetic_link.to_vec(),
			asset_id,
			currency.clone(),
			prices.clone(),
			additional_prices.clone(),
			*total_price,
			now,
		);
		Self::insert_genetic_analysis_order_to_storage(&genetic_analysis_order);

		Ok(genetic_analysis_order)
	}

	fn cancel_genetic_analysis_order(
		customer_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		let genetic_analysis_order = GeneticAnalysisOrders::<T>::get(genetic_analysis_order_id)
			.ok_or(Error::<T>::GeneticAnalysisOrderNotFound)?;

		if &genetic_analysis_order.customer_id != customer_id {
			return Err(Error::<T>::UnauthorizedGeneticAnalysisOrderCancellation)
		}

		let tracking_id = &genetic_analysis_order.genetic_analysis_tracking_id;
		let _ = T::GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(tracking_id)
			.filter(|genetic_analysis| genetic_analysis.is_registered())
			.ok_or(Error::<T>::OngoingGeneticAnalysisOrderCannotBeCancelled)?;

		// Default status would be cancelled
		let mut genetic_analysis_order_status = GeneticAnalysisOrderStatus::Cancelled;

		if genetic_analysis_order.status == GeneticAnalysisOrderStatus::Paid {
			let total_price = genetic_analysis_order.total_price;

			if !Self::is_pallet_balance_sufficient_for_transfer(total_price) {
				return Err(Error::<T>::InsufficientPalletFunds)
			}

			Self::do_transfer(
				&genetic_analysis_order.currency,
				&Self::account_id(),
				customer_id,
				total_price,
				false,
				genetic_analysis_order.asset_id,
			)?;

			// If code reaches here change status to Refunded
			genetic_analysis_order_status = GeneticAnalysisOrderStatus::Refunded;
		}

		Self::remove_genetic_analysis_order_id_from_pending_genetic_analysis_orders_by_seller(
			&genetic_analysis_order.seller_id,
			genetic_analysis_order_id,
		);

		// Delete dna sample associated with the genetic_analysis_order
		let _ = T::GeneticAnalysis::delete_genetic_analysis(tracking_id);
		let genetic_analysis_order = Self::update_genetic_analysis_order_status(
			genetic_analysis_order_id,
			genetic_analysis_order_status,
		)?;

		Ok(genetic_analysis_order)
	}

	fn set_genetic_analysis_order_paid(
		account_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		let genetic_analysis_order = GeneticAnalysisOrders::<T>::get(genetic_analysis_order_id)
			.ok_or(Error::<T>::GeneticAnalysisOrderNotFound)?;

		if account_id != &genetic_analysis_order.customer_id {
			let _ = EscrowKey::<T>::get()
				.filter(|admin| admin == account_id)
				.ok_or(Error::<T>::Unauthorized)?;
		}

		let customer_id = &genetic_analysis_order.customer_id;
		let total_price = genetic_analysis_order.total_price;

		if !Self::is_balance_sufficient_for_payment(customer_id, total_price) {
			return Err(Error::<T>::InsufficientFunds)
		}

		Self::do_transfer(
			&genetic_analysis_order.currency,
			customer_id,
			&Self::account_id(),
			total_price,
			true,
			genetic_analysis_order.asset_id, // Set AssetId
		)?;

		let genetic_analysis_order = Self::update_genetic_analysis_order_status(
			genetic_analysis_order_id,
			GeneticAnalysisOrderStatus::Paid,
		)?;

		Ok(genetic_analysis_order)
	}

	fn fulfill_genetic_analysis_order(
		escrow_account_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		// Only the admin can fulfill the genetic_analysis_order
		let _ = EscrowKey::<T>::get()
			.filter(|admin| admin == escrow_account_id)
			.ok_or(Error::<T>::Unauthorized)?;

		let genetic_analysis_order = GeneticAnalysisOrders::<T>::get(genetic_analysis_order_id)
			.ok_or(Error::<T>::GeneticAnalysisOrderNotFound)?;

		let tracking_id = &genetic_analysis_order.genetic_analysis_tracking_id;
		let _ = T::GeneticAnalysis::genetic_analysis_by_genetic_analysis_tracking_id(tracking_id)
			.filter(|genetic_analysis| genetic_analysis.process_success())
			.ok_or(Error::<T>::GeneticAnalysisNotSuccessfullyProcessed)?;

		if !Self::is_pallet_balance_sufficient_for_transfer(genetic_analysis_order.total_price) {
			return Err(Error::<T>::InsufficientPalletFunds)
		}

		// Calculate 5% of the price_component_value
		let mut price_component_substracted_value: BalanceOf<T> = 0u128.saturated_into();
		for analysis_order in genetic_analysis_order.prices.iter() {
			price_component_substracted_value += analysis_order.value / 20u128.saturated_into();
		}

		// 5% of the price_component_value is substracted
		let total_price_paid =
			genetic_analysis_order.total_price - price_component_substracted_value;

		// Withhold 5% for DBIO
		Self::do_transfer(
			&genetic_analysis_order.currency,
			&Self::account_id(),
			&genetic_analysis_order.seller_id,
			total_price_paid,
			true,
			genetic_analysis_order.asset_id, // Set AssetId
		)?;

		// Transfer 5% to DBIO Treasury
		Self::do_transfer(
			&genetic_analysis_order.currency,
			&Self::account_id(),
			&TreasuryKey::<T>::get().unwrap(),
			price_component_substracted_value,
			false,
			genetic_analysis_order.asset_id, // Set AssetId
		)?;

		let genetic_analysis_order = Self::update_genetic_analysis_order_status(
			genetic_analysis_order_id,
			GeneticAnalysisOrderStatus::Fulfilled,
		)?;

		Ok(genetic_analysis_order)
	}

	fn set_genetic_analysis_order_refunded(
		escrow_account_id: &T::AccountId,
		genetic_analysis_order_id: &T::Hash,
	) -> Result<Self::GeneticAnalysisOrder, Self::Error> {
		if escrow_account_id.clone() != EscrowKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		let genetic_analysis_order = GeneticAnalysisOrders::<T>::get(genetic_analysis_order_id)
			.ok_or(Error::<T>::GeneticAnalysisOrderNotFound)?;

		let tracking_id = &genetic_analysis_order.genetic_analysis_tracking_id;
		if !Self::genetic_analysis_order_can_be_refunded(tracking_id) {
			return Err(Error::<T>::GeneticAnalysisOrderNotYetExpired)
		}

		if !Self::is_pallet_balance_sufficient_for_transfer(genetic_analysis_order.total_price) {
			return Err(Error::<T>::InsufficientPalletFunds)
		}

		// Transfer 5% to DBIO Treasury
		Self::do_transfer(
			&genetic_analysis_order.currency,
			&Self::account_id(),
			&genetic_analysis_order.customer_id,
			genetic_analysis_order.total_price,
			false,
			genetic_analysis_order.asset_id, // Set AssetId
		)?;

		let genetic_analysis_order = Self::update_genetic_analysis_order_status(
			genetic_analysis_order_id,
			GeneticAnalysisOrderStatus::Refunded,
		)?;

		Ok(genetic_analysis_order)
	}

	fn update_escrow_key(
		account_id: &T::AccountId,
		escrow_key: &T::AccountId,
	) -> Result<(), Self::Error> {
		if account_id.clone() != EscrowKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		EscrowKey::<T>::put(escrow_key);

		Ok(())
	}

	fn update_treasury_key(
		account_id: &T::AccountId,
		treasury_key: &T::AccountId,
	) -> Result<(), Self::Error> {
		if account_id.clone() != TreasuryKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		TreasuryKey::<T>::put(treasury_key);

		Ok(())
	}

	fn is_pending_genetic_analysis_order_ids_by_seller_exist(account_id: &T::AccountId) -> bool {
		match PendingGeneticAnalysisOrdersBySeller::<T>::get(account_id) {
			Some(_arr) => !_arr.is_empty(),
			None => false,
		}
	}
}

impl<T: Config> GeneticAnalysisOrderEventEmitter<T> for Pallet<T> {
	fn emit_event_genetic_analysis_order_failed(genetic_analysis_order_id: &HashOf<T>) {
		match Self::genetic_analysis_order_by_id(genetic_analysis_order_id) {
			None => Self::deposit_event(Event::GeneticAnalysisOrderNotFound),
			Some(genetic_analysis_order) =>
				Self::deposit_event(Event::GeneticAnalysisOrderFailed(genetic_analysis_order)),
		}
	}
}

impl<T: Config> GeneticAnalysisOrderStatusUpdater<T> for Pallet<T> {
	fn update_status_failed(genetic_analysis_order_id: &HashOf<T>) {
		match Self::genetic_analysis_order_by_id(genetic_analysis_order_id) {
			None => Self::deposit_event(Event::GeneticAnalysisOrderNotFound),
			Some(genetic_analysis_order) => {
				let result = Self::update_genetic_analysis_order_status(
					&genetic_analysis_order.id,
					GeneticAnalysisOrderStatus::Failed,
				);

				if result.is_err() {
					Self::deposit_event(Event::GeneticAnalysisOrderNotFound)
				}
			},
		}
	}

	fn remove_genetic_analysis_order_id_from_pending_genetic_analysis_orders_by_seller(
		seller_id: &AccountIdOf<T>,
		genetic_analysis_order_id: &HashOf<T>,
	) {
		Self::remove_genetic_analysis_order_id_from_pending_genetic_analysis_orders_by_seller(
			seller_id,
			genetic_analysis_order_id,
		);
	}

	fn is_pending_genetic_analysis_order_by_seller_exist(seller_id: &AccountIdOf<T>) -> bool {
		Self::is_pending_genetic_analysis_order_ids_by_seller_exist(seller_id)
	}
}