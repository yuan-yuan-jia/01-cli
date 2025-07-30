use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_text_key_generate, process_text_sign, process_text_verify, Base64SubCommand, Opts,
    SubCommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("password:{password}");
            let estimate = zxcvbn::zxcvbn(password.as_str(), &[]);
            eprintln!("Password strength: {}", estimate.score());
        }
        SubCommand::Base64(base64_sub_command) => match base64_sub_command {
            Base64SubCommand::Encode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let ret = process_encode(&mut reader, opts.format)?;
                println!("{}", ret);
            }
            Base64SubCommand::Decode(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let ret = process_decode(&mut reader, opts.format)?;
                println!("{}", ret);
            }
        },
        SubCommand::Text(text_sub_command) => match text_sub_command {
            rcli::TextSubCommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sign = process_text_sign(&mut reader, &key, opts.format)?;
                let encoded = BASE64_URL_SAFE_NO_PAD.encode(sign);
                println!("{encoded}");
            }
            rcli::TextSubCommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = BASE64_URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
            rcli::TextSubCommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    std::fs::write(opts.output_path.join(k), v)?;
                }
            }
        },
    }
    Ok(())
}
