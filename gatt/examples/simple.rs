use gatt::characteristics as ch;
use gatt::services as srv;
use gatt::{CharacteristicProperties, Registration, Server};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Token {
    DeviceName,
    BatteryLevelNotify,
}

fn new_registration() -> Registration<Token> {
    let mut registration = Registration::new();

    registration.add_primary_service(srv::GENERIC_ACCESS);
    registration.add_characteristic_with_token(
        Token::DeviceName,
        ch::DEVICE_NAME,
        "abc",
        CharacteristicProperties::WRITE,
    );
    registration.add_characteristic(
        ch::APPEARANCE,
        0x03c0u16.to_le_bytes().to_vec(),
        CharacteristicProperties::READ,
    );

    registration.add_primary_service(srv::GENERIC_ATTRIBUTE);
    registration.add_characteristic(ch::SERVICE_CHANGED, "", CharacteristicProperties::INDICATE);

    registration.add_primary_service(srv::DEVICE_INFORMATION);
    registration.add_characteristic(
        ch::MANUFACTURER_NAME_STRING,
        "機械",
        CharacteristicProperties::READ,
    );
    registration.add_characteristic(
        ch::MODEL_NUMBER_STRING,
        "A123",
        CharacteristicProperties::READ,
    );
    registration.add_characteristic(
        ch::SERIAL_NUMBER_STRING,
        "333-444",
        CharacteristicProperties::READ,
    );

    registration.add_primary_service(srv::BATTERY);
    registration.add_characteristic_with_token(
        Token::BatteryLevelNotify,
        ch::BATTERY_LEVEL,
        "",
        CharacteristicProperties::NOTIFY,
    );

    registration
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use std::io::stdin;
    use tokio::task::spawn_blocking;

    env_logger::init();

    let server = Server::bind()?;
    let connection = server.accept().await?;
    let (task, outgoing, mut events) = connection.run(false, new_registration());
    let mut task = tokio::spawn(task);

    let mut n = 0;
    loop {
        tokio::select! {
            r = Pin::new(&mut task) => r??,

            maybe_line = spawn_blocking(|| stdin().read_line(&mut String::new())) => {
                maybe_line??;
                outgoing.notify(&Token::BatteryLevelNotify, vec![n])?;
                n += 1;
            }

            event = events.next() => {
                if let Some(event) = event {
                    println!("{:?}", event);
                }
            }
        }
    }
}
