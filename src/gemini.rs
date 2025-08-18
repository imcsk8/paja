use gemini_rust::Gemini;

/// Generate output
pub async fn generate(
    //TODO: input: Form<UserInput<'_>>,
    input: String,
    //TODO: this is for google docs token: GoogleToken,
//TODO) -> Result<Box<dyn rocket::response::Responder + 'static>, String> {
) -> Result<String, String> {
    let gemini_api_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set.");

    // 1. Craft the prompt for Gemini (no changes)
    let prompt = format!(
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
    );

    // 2. Call Gemini API using the new, refactored function
    let cv_content = match call_gemini(&prompt, &gemini_api_key).await {
        Ok(text) => text,
        Err(e) => return Err(format!("Error interacting with the AI model: {}", e)),
    };

    Ok(cv_content)

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
async fn call_gemini(prompt: &str, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 1. Initialize the client with the API key.
    let client = Gemini::new(api_key.to_string());

    // 2. Send the content generation request.
    // The crate handles building the request body.
    let response = client.generate_content()
        .with_system_prompt(prompt)
        .with_user_message("Generando CV")
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
