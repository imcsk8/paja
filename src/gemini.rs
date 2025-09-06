use gemini_rust::Gemini;
use rocket::serde::{ Deserialize, Serialize };
use rocket;


///  Gemini configiration
#[derive(Serialize, Deserialize)]
pub struct GeminiConfig {
    pub api_key: String,
}


/// Prompt sent by the user
#[derive(Serialize, Deserialize)]
pub struct PromptPayload {
    pub content: String,
}


/// Generate gemini output
pub async fn generate(
    //TODO: input: Form<UserInput<'_>>,
    persona: &str,
    input: String,
    gemini_api_key: String,
    //TODO: this is for google docs token: GoogleToken,
//TODO) -> Result<Box<dyn rocket::response::Responder + 'static>, String> {
) -> Result<String, String> {

    // 1. Craft the prompt for Gemini (no changes)
    /* TODO: check if needed let prompt = format!(
        "Based on the following user-provided information, generate a professional Curriculum Vitae (CV).
        The output should be in clean Markdown format, with sections for Summary, Work Experience, Education, and Skills.
        Do not include any text other than the Markdown CV itself. The content should be in the language of the user-provided
        information.

        User Information:
        ---
        {}
        ---",
        //TODO: input.raw_text
        input
    );*/

    // 2. Call Gemini API using the new, refactored function
    let content = match call_gemini(&persona, &input, &gemini_api_key).await {
        Ok(t) => t,
        Err(e) => return Err(format!("Error interacting with the AI model: {}", e)),
    };

    Ok(content)

    // 3. Generate the requested output format (no changes)
   /*TODO match input.output_format {
        "pdf" => {
            let pdf_data = generate_pdf(&cv_content).map_err(|e| format!("PDF generation failed: {}", e))?;
            Ok(Box::new(content::Raw("application/pdf", pdf_data)))
        },
        "gdoc" => {
            let doc_url = create_google_doc(&cv_content, &token.0).await.map_err(|e| format!("Google Doc creation failed: {}", e))?;
            Ok(Box::new(Redirect::to(doc_url)))
        },
        _ => Err("Invalid output format specified.".to_string()),
    }*/
}


/// Handle the gemini API calls
async fn call_gemini(persona: &str, prompt: &str, api_key: &str)
-> Result<String, Box<dyn std::error::Error>> {
    // 1. Initialize the client with the API key.
    let client = Gemini::new(api_key.to_string());

    // 2. Send the content generation request.
    // The crate handles building the request body.
    let response = client.generate_content()
        .with_system_instruction(persona)
        //.with_system_prompt(prompt)
        .with_user_message(prompt)
        .execute()
        .await?;

    // 3. Extract the text content in a safe way.
    // The crate provides the response structs for us.
    /* TODO CHECK: let text = response
        .candidates
        .get(0)
        .and_then(|c| c.content.parts.get(0))
        .map(|p| p.text.clone())
        .ok_or("No content found in Gemini response")?;
    */

    Ok(response.text())
}


/// Get the gemini API key from the configuration file
pub fn get_gemini_api_key() -> String {
    let figment = rocket::Config::figment();

    // Extract JWT configuration from the "jwt" table in Rocket.toml
    let gemini: GeminiConfig = figment
        .extract_inner("gemini")
        .expect("Gemini API key missing from Rocket.toml");
    gemini.api_key
}
