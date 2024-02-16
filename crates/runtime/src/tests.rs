#[test]
fn force_adjust_total_issuance_example() {
	ExtBuilder::default().build_and_execute_with(|| {
		// First we set the TotalIssuance to 64 by giving Alice a balance of 64.
		assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), ALICE, 64));
		let old_ti = Balances::total_issuance();
		assert_eq!(old_ti, 64, "TI should be 64");

		// Now test the increase:
		assert_ok!(Balances::force_adjust_total_issuance(RawOrigin::Root.into(), Inc, 32));
		let new_ti = Balances::total_issuance();
		assert_eq!(old_ti + 32, new_ti, "Should increase by 32");

		// If Alice tries to call it, it errors:
		assert_noop!(
			Balances::force_adjust_total_issuance(RawOrigin::Signed(ALICE).into(), Inc, 32),
			BadOrigin,
		);
	});
}