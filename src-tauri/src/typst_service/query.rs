use comemo::Track;
use ecow::{eco_format, EcoString};
use serde::Serialize;
use typst::diag::{bail, HintedStrResult, StrResult, Warned};
use typst::foundations::{Content, IntoValue, LocatableSelector, Scope};
use typst::layout::PagedDocument;
use typst::syntax::Span;
use typst::World;
use typst_eval::{eval_string, EvalMode};


use super::world::SystemWorld;
use super::args::{QueryArgs, SerializationFormat};
use super::compiler::print_diagnostics;
use super::super::util::set_failed;

pub fn query(config: &QueryArgs) -> HintedStrResult<()> {
    let mut world = SystemWorld::new(&config.input, &config.world, &config.process)?;

    // Reset everything and ensure that the main file is present.
    world.reset();
    world.source(world.main()).map_err(|err| err.to_string())?;

    let Warned { output, warnings } = typst::compile(&world);

    match output {
        // Retrieve and print query results.
        Ok(document) => {
            let data = retrieve(&world, config, &document)?;
            let serialized = format(data, config)?;
            println!("{serialized}");
            print_diagnostics(&world, &[], &warnings, config.process.diagnostic_format)
                .map_err(|err| eco_format!("failed to print diagnostics ({err})"))?;
        }

        // Print diagnostics.
        Err(errors) => {
            set_failed();
            print_diagnostics(&world, &errors, &warnings, config.process.diagnostic_format)
                .map_err(|err| eco_format!("failed to print diagnostics ({err})"))?;
        }
    }

    Ok(())
}

/// Retrieve the matches for the selector.
fn retrieve(
    world: &dyn World,
    config: &QueryArgs,
    document: &PagedDocument,
) -> HintedStrResult<Vec<Content>> {
    let selector = eval_string(
        &typst::ROUTINES,
        world.track(),
        &config.selector,
        Span::detached(),
        EvalMode::Code,
        Scope::default(),
    )
    .map_err(|errors| {
        let mut message = EcoString::from("failed to evaluate selector");
        for (i, error) in errors.into_iter().enumerate() {
            message.push_str(if i == 0 { ": " } else { ", " });
            message.push_str(&error.message);
        }
        message
    })?
    .cast::<LocatableSelector>()?;

    Ok(document
        .introspector
        .query(&selector.0)
        .into_iter()
        .collect::<Vec<_>>())
}

/// Format the query result in the output format.
fn format(elements: Vec<Content>, config: &QueryArgs) -> StrResult<String> {
    if config.one && elements.len() != 1 {
        bail!("expected exactly one element, found {}", elements.len());
    }

    let mapped: Vec<_> = elements
        .into_iter()
        .filter_map(|c| match &config.field {
            Some(field) => c.get_by_name(field).ok(),
            _ => Some(c.into_value()),
        })
        .collect();

    if config.one {
        let Some(value) = mapped.first() else {
            bail!("no such field found for element");
        };
        serialize(value, config.format, config.pretty)
    } else {
        serialize(&mapped, config.format, config.pretty)
    }
}

/// Serialize data to the output format.
fn serialize(
    data: &impl Serialize,
    format: SerializationFormat,
    pretty: bool,
) -> StrResult<String> {
    match format {
        SerializationFormat::Json => {
            if pretty {
                serde_json::to_string_pretty(data).map_err(|e| eco_format!("{e}"))
            } else {
                serde_json::to_string(data).map_err(|e| eco_format!("{e}"))
            }
        }
        SerializationFormat::Toml => toml::to_string(data).map_err(|e| eco_format!("{e}")),
    }
}
