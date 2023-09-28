mod bundler;
mod templating;

fn main() {
    bundler::bundle().unwrap();
}
