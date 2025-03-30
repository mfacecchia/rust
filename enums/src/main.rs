// NOTE: An enum is like a struct but is used for
// pre-defined values from the developer.
// Like in this example where the status can be one of the provided values.
#[derive(Debug)]
enum PaymentStatus {
    PROCESSED(StatusCodes),
    PENDING(StatusCodes),
    REFUSED(StatusCodes),
}

#[derive(Debug)]
enum StatusCodes {
    P200(String),
    P300(String),
    P400(String),
}

fn main() {
    // NOTE: In Rust, the Null pointer does not exist (thank you Rust),
    // but a variable can still have a value representing "nothing" through the `Option<T>` type.
    // This special enum can handle two values; `None`, and `Some`.
    // If the value has the `Some` value, then we will need to specify the value the variable will handle,
    // otherwise we can just assign to the variable the type `Option::None` and call it a day.
    // Before any operation with the data, we will still need to obtain the data from `Some` (if present),
    // ensuring type and memory safety.
    let payment: Option<PaymentStatus> = Option::Some(PaymentStatus::PENDING(StatusCodes::P300(
        String::from("Pending"),
    )));

    let final_status = process_payment(payment);
    println!("Status: {final_status:#?}");

    let no_payment = Option::None;
    process_payment(no_payment);
}

fn process_payment(payment: Option<PaymentStatus>) -> PaymentStatus {
    // TODO: May want to refactor this (don't like too many nestings :/)
    match payment {
        Option::None => {
            println!("Could not process the payment. Aborting.");
            return PaymentStatus::REFUSED(StatusCodes::P400(String::from("Unknown error.")));
        }
        Option::Some(payment) => {
            match payment {
                PaymentStatus::REFUSED(sc) => match sc {
                    StatusCodes::P200(msg) | StatusCodes::P300(msg) | StatusCodes::P400(msg) => {
                        println!("Payment status: {msg}");
                        println!("Payment processed successfully");
                        return PaymentStatus::REFUSED(StatusCodes::P400(String::from(
                            "Invalid payment method.",
                        )));
                    }
                },
                // Binding `payment` values to `msg` in order for it to be used
                PaymentStatus::PENDING(sc) | PaymentStatus::PROCESSED(sc) => match sc {
                    StatusCodes::P200(msg) | StatusCodes::P300(msg) | StatusCodes::P400(msg) => {
                        println!("Payment status: {msg}");
                        println!("Payment processed successfully");
                        return PaymentStatus::PROCESSED(StatusCodes::P200(String::from(
                            "Success: true",
                        )));
                    }
                },
            }
        }
    }
}
