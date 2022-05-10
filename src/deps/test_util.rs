use crate::deps::collect_ast_dependencies;
use crate::deps::Dependency;
use crate::util::ParsingContext;

#[cfg(test)]

pub fn deps(source: &str) -> Result<Vec<Dependency>, crate::error::Error> {
    let ast = rnix::parse(source);
    let context = ParsingContext::new("./test.nix", source);
    return collect_ast_dependencies(&context, ast.node());
}
