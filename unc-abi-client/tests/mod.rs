use std::fs;

use quote::quote;

use unc_abi_client::Generator;

#[test]
fn test_generate_abi() -> anyhow::Result<()> {
    let tmp_dir = tempfile::tempdir()?;
    let tmp_dir_path = tmp_dir.into_path();
    Generator::new(tmp_dir_path.clone())
        .file("tests/adder.json")
        .generate()?;

    let generated_code = fs::read_to_string(tmp_dir_path.join("adder.rs"))?;
    let expected = quote! {
        pub type Pair = Vec<i64>;
        pub struct AbiClient {
            pub contract: utility_workspaces::Contract,
        }
        impl AbiClient {
            pub async fn add(
                &self,
                a: Pair,
                b: Pair
            ) -> anyhow::Result<Pair> {
                let result = self
                    .contract
                    .call("add")
                    .args_json([a, b])
                    .view()
                    .await?;
                Ok(result.json::<Pair>()?)
            }
        }
    };
    let syntax_tree = syn::parse_file(&expected.to_string()).unwrap();
    let expected = prettyplease::unparse(&syntax_tree);
    assert_eq!(expected, generated_code);

    Ok(())
}
