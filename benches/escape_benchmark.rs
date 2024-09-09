use atom_syndication_format::escape_xml;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

static INPUT_SHORT_TEXT: &'static str = "<div></div>";

static INPUT_LONG_TEXT: &'static str = r#"
<pre><div class="buttons"><button class="fa fa-copy clip-button" title="Copy to clipboard" aria-label="Copy to clipboard">
<i class="tooltiptext"></i></button></div><code class="language-rust ignore hljs">&amp;<span class="hljs-built_in">i32</span>
<span class="hljs-comment">// a reference</span> &amp;<span class="hljs-symbol">'a</span> <span class="hljs-built_in">i32</span>
// <span class="hljs-comment">// a reference with an explicit lifetime</span> &amp;<span class="hljs-symbol">'a</span> 
<span class="hljs-keyword">mut</span> <span class="hljs-built_in">i32</span> <span class="hljs-comment">// a mutable reference with an explicit lifetime</span></code></pre>
"#;

static INPUT_NO_ESCAPE : &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec porta, 
                       libero eget aliquet luctus, sapien justo dapibus dui, non tristique tortor nulla volutpat dolor. 
                       Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. 
                       Vestibulum ut urna est. Praesent interdum dui leo, nec convallis ipsum maximus sed. Maecenas 
                       ornare pharetra lectus sit amet viverra. In hac habitasse platea dictumst. Integer nec nisi feugiat,
                       sagittis magna quis, dignissim est. ";

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("escape short text", |b| {
    b.iter(|| escape_xml!(black_box(&INPUT_SHORT_TEXT)))
  });

  c.bench_function("escape long text", |b| {
    b.iter(|| escape_xml!(black_box(&INPUT_LONG_TEXT)))
  });

  c.bench_function("non-escape text", |b| {
    b.iter(|| escape_xml!(black_box(&INPUT_NO_ESCAPE)))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
