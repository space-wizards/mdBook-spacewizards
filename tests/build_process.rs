mod dummy_book;

use crate::dummy_book::DummyBook;
use mdbook_spacewizards::book::Book;
use mdbook_spacewizards::build_opts::BuildOpts;
use mdbook_spacewizards::config::Config;
use mdbook_spacewizards::errors::*;
use mdbook_spacewizards::preprocess::{Preprocessor, PreprocessorContext};
use mdbook_spacewizards::renderer::{RenderContext, Renderer};
use mdbook_spacewizards::MDBook;
use std::sync::{Arc, Mutex};

struct Spy(Arc<Mutex<Inner>>);

#[derive(Debug, Default)]
struct Inner {
    run_count: usize,
    rendered_with: Vec<String>,
}

impl Preprocessor for Spy {
    fn name(&self) -> &str {
        "dummy"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        let mut inner = self.0.lock().unwrap();
        inner.run_count += 1;
        inner.rendered_with.push(ctx.renderer.clone());
        Ok(book)
    }
}

impl Renderer for Spy {
    fn name(&self) -> &str {
        "dummy"
    }

    fn render(&self, _ctx: &RenderContext) -> Result<()> {
        let mut inner = self.0.lock().unwrap();
        inner.run_count += 1;
        Ok(())
    }
}

#[test]
fn mdbook_runs_preprocessors() {
    let spy: Arc<Mutex<Inner>> = Default::default();

    let temp = DummyBook::new().build().unwrap();
    let cfg = Config::default();
    let build_opts = BuildOpts::default();

    let mut book = MDBook::load_with_config(temp.path(), cfg, build_opts).unwrap();
    book.with_preprocessor(Spy(Arc::clone(&spy)));
    book.build().unwrap();

    let inner = spy.lock().unwrap();
    assert_eq!(inner.run_count, 1);
    assert_eq!(inner.rendered_with.len(), 1);
    assert_eq!(
        "html", inner.rendered_with[0],
        "We should have been run with the default HTML renderer"
    );
}

#[test]
fn mdbook_runs_renderers() {
    let spy: Arc<Mutex<Inner>> = Default::default();

    let temp = DummyBook::new().build().unwrap();
    let cfg = Config::default();
    let build_opts = BuildOpts::default();

    let mut book = MDBook::load_with_config(temp.path(), cfg, build_opts).unwrap();
    book.with_renderer(Spy(Arc::clone(&spy)));
    book.build().unwrap();

    let inner = spy.lock().unwrap();
    assert_eq!(inner.run_count, 1);
}
