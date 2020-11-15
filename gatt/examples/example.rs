use gatt::{CharacteristicProperties, Registration, Server, Uuid};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Token {
    DeviceName,
    ServiceChanged,
    BatteryLevelNotify,
}

fn new_registration() -> Registration<Token> {
    let mut registration = Registration::new();

    registration.add_primary_service(Uuid::new_uuid16(0x1800));
    registration.add_characteristic_with_token(
        Token::DeviceName,
        Uuid::new_uuid16(0x2A00),
        "abc",
        CharacteristicProperties::WRITE,
    );
    registration.add_characteristic(
        Uuid::new_uuid16(0x2A01),
        0x03c0u16.to_le_bytes().to_vec(),
        CharacteristicProperties::READ,
    );

    registration.add_primary_service(Uuid::new_uuid16(0x1801));
    registration.add_characteristic_with_token(
        Token::ServiceChanged,
        Uuid::new_uuid16(0x2A05),
        "",
        CharacteristicProperties::INDICATE,
    );

    registration.add_primary_service(Uuid::new_uuid16(0x180A));
    registration.add_characteristic(
        Uuid::new_uuid16(0x2A29),
        "機械",
        CharacteristicProperties::READ,
    );
    registration.add_characteristic(
        Uuid::new_uuid16(0x2A24),
        "A123",
        CharacteristicProperties::READ,
    );
    registration.add_characteristic(
        Uuid::new_uuid16(0x2A25),
        "333-444",
        CharacteristicProperties::READ,
    );

    registration.add_primary_service(Uuid::new_uuid16(0x180F));
    registration.add_characteristic_with_token(
        Token::BatteryLevelNotify,
        Uuid::new_uuid16(0x2A19),
        "",
        CharacteristicProperties::NOTIFY,
    );

    registration
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use std::io::stdin;
    use tokio::task::spawn_blocking;

    let server = Server::bind()?;
    let connection = server.accept().await?;
    let (t, o, mut e) = connection.run(new_registration());
    let mut t = tokio::spawn(t);

    let mut n = 0;
    loop {
        tokio::select! {
            _ = Pin::new(&mut t) => break,

            maybe_line = spawn_blocking(|| stdin().read_line(&mut String::new())) => {
                maybe_line??;
                o.notify(&Token::BatteryLevelNotify, vec![n])?;
                //ooutbound.indicate(0x000E.into(), vec![0x0C, 0x00, 0x0F, 0x00].into()).await?; // GATT / Service Changed
                n += 1;
            }

            event = e.next() => {
                println!("{:?}", event);
            }
        }
    }
    Ok(())
}
