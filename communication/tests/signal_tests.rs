use communication::signal::*;
use communication::slot::*;
use communication_attributes::signal;
use mockall::predicate::*;

#[test]
fn test_slot_mock_called_one_arg() {
    #[signal(i32)]
    struct ClickSignal;

    let mut mock = MockSlot::<i32>::new();

    mock.expect_call().with(eq(42)).times(1).return_const(());

    let slot: SlotRef<i32> = std::rc::Rc::new(std::cell::RefCell::new(mock));

    let mut signal = ClickSignal::new();

    let _id = signal.connect(slot.clone());
    signal.emit(&42);
}

#[test]
#[ignore = "Bug in feature"]
fn test_slot_mock_called_two_arg() {
    // #[signal(i32, bool)]
    // struct ClickSignal;

    // let mut mock = MockSlot::<i32, bool>::new();

    // mock.expect_call()
    //     .withf(|a, b| *a == 42 && *b == true)
    //     .times(1)
    //     .return_const(());

    // let slot = std::rc::Rc::new(std::cell::RefCell::new(mock));

    // let mut signal = ClickSignal::new();

    // let _id = signal.connect(slot.clone());
    // signal.emit(&42, &true);
}
