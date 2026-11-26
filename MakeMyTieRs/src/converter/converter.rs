pub fn create_test_asset() {

}

fn get_image_hash(string: path) {
    let img = image::open(path)?;

    let bytes = std::fs::read(img)?;
    let ext = image::guess_format(&bytes)?.extensions()[0];

    
}