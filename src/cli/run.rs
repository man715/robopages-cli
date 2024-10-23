use std::{collections::BTreeMap, sync::Arc};

use camino::Utf8PathBuf;

use crate::{
    book::{openai, Book},
    runtime::{self, prompt},
};

pub(crate) async fn run(path: Utf8PathBuf, func_name: String, auto: bool) -> anyhow::Result<()> {
    let book = Arc::new(Book::from_path(path, None)?);
    let function = book.get_function(&func_name)?;

    let mut arguments = BTreeMap::new();

    for arg_name in function.function.parameters.keys() {
        arguments.insert(
            arg_name.to_string(),
            prompt::ask(
                &format!(">> enter value for argument '{}': ", arg_name),
                &[],
            )?,
        );
    }

    let call = openai::Call {
        id: None,
        function: openai::FunctionCall {
            name: func_name,
            arguments,
        },
        call_type: "function".to_string(),
    };

    log::debug!("running function {:?}", function);

    let result = runtime::execute_call(!auto, 10, book, call).await?;

    println!("\n{}", result.content);

    Ok(())
}
