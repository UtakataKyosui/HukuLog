use loco_rs::prelude::*;
use webauthn_rs::prelude::*;

pub fn create_webauthn(_ctx: &AppContext) -> Result<Webauthn> {
    let rp_id = "localhost"; // In production, this should be your domain
    let rp_origin = Url::parse("http://localhost:5150")
        .map_err(|e| Error::BadRequest(format!("Invalid URL: {}", e)))?;
    let builder = WebauthnBuilder::new(rp_id, &rp_origin)
        .map_err(|e| Error::BadRequest(format!("WebAuthn builder error: {}", e)))?;
    
    let webauthn = builder
        .rp_name("HukuLog")
        .build()
        .map_err(|e| Error::BadRequest(format!("WebAuthn build error: {}", e)))?;
    
    Ok(webauthn)
}