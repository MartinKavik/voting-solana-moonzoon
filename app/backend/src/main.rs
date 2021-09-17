use moon::*;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Voting")
        .append_to_head("
            <style>
                body {
                    background: black;
                }
            </style>
        ")
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[moon::main]
async fn main() -> std::io::Result<()> {
    println!("Voting owner keypair: {}", include_str!("../../../program/keypairs/voting-owner-keypair.json"));
    println!("Program pubkey: {}", include_str!("../../../program/keypairs/program-pubkey"));
    start(frontend, up_msg_handler, |_| {}).await
}
