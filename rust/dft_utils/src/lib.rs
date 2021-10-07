mod principal;
mod tx_id;

use candid::IDLProg;
pub use principal::*;
pub use tx_id::*;

pub fn is_support_interface(did: String, interface_sig: String) -> bool {
  let verify_service_desc = format!("service:{{ {0};}}", interface_sig);
  let verify_ast_result = verify_service_desc.parse::<IDLProg>();

  match verify_ast_result {
      Ok(verify_ast) => {
          let verify_pretty: String = candid::parser::types::to_pretty(&verify_ast, 80);
          let verify_pretty_sub: String =
              verify_pretty.replace("service : { ", "").replace(" }", "");

          let origin_did = did;
          let origin_ast: IDLProg = origin_did.parse().unwrap();
          let origin_pretty: String = candid::parser::types::to_pretty(&origin_ast, 80);
          origin_pretty.contains(&verify_pretty_sub)
      }
      _ => false,
  }
}