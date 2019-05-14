extern crate base;
extern crate core;

mod options;

use base::chrono;
use base::math::int2;
use base::thread;
use core::image;
use core::rendering::driver;
use core::resource::{Cache, TypedCache};
use core::scene::material::Material;
use core::scene::material::Provider as MaterialProvider;
use core::scene::{self, Scene};
use core::take;
use options::Options;
use std::env;
use std::time::Instant;

fn main() {
    let total_start = Instant::now();

    println!("Welcome to bud ()!");

    let args: Vec<String> = env::args().collect();

    let options = Options::new(&args);

    let available_threads = thread::available_threads();

    let num_workers;

    if options.threads <= 0 {
        let num_threads = available_threads as i32 + options.threads;
        num_workers = num_threads.max(1) as u32;
    } else {
        num_workers = available_threads.min(options.threads.max(1) as u32);
    }

    println!("#Threads {}", num_workers);

    let mut thread_pool = thread::Pool::new(num_workers);

    let mut scene_loader = scene::Loader::new();

    {
        let mut file_system = scene_loader.resource_manager().file_system();

        if options.mounts.is_empty() {
            file_system.push_mount("../data/");
        }
    }

    println!("Loading...");

    let loading_start = Instant::now();

    let stream = scene_loader.resource_manager().file_system().read_stream(&options.take);

    if let Err(err) = stream {
        println!("Loading take \"{}\": {}", options.take, err.message());
        std::process::exit(1);
    }

    let take = take::Loader::load(&mut stream.unwrap());

    //   thread_pool.run_parallel();

    if let Err(err) = take {
        println!("Loading take \"{}\": {}", options.take, err.message());
        std::process::exit(1);
    }

    let imagely = image::Float4::new(int2::new(16, 16));

    scene_loader.resource_manager().stuff(&imagely);

    //   let material_provider = Box::new(MaterialProvider {});
    //   let mut material_cache = Box::new(TypedCache::<Material>::new(material_provider));

    // if let Err(err) = material_cache.load("stuff.material", scene_loader.resource_manager()) {
    //     println!("{}", err.message());
    // }

    //    scene_loader.resource_manager().register(material_cache);

    if let Err(err) = scene_loader.resource_manager().load_material("stuff.material") {
        println!("{}", err.message());
    }

    let mut take = take.unwrap();

    let mut scene = Scene::new();

    if let Err(err) = scene_loader.load(&take.scene_filename, &mut scene) {
        println!("Loading take \"{}\": {}", take.scene_filename, err.message());
        std::process::exit(1);
    }

    println!("Loading time {} s", chrono::duration_to_seconds(loading_start.elapsed()));

    println!("Rendering...");

    let rendering_start = Instant::now();
    {
        let mut driver = driver::FinalFrame::new(&thread_pool, &take);

        driver.render(&scene, &mut take.view, &mut take.exporters);
    }

    //   thread_pool.run_parallel();

    println!("Total render time {} s", chrono::duration_to_seconds(rendering_start.elapsed()));

    println!("Total elapsed time {} s", chrono::duration_to_seconds(total_start.elapsed()));
}
