use rand::prelude::*;
use criterion::{ criterion_group, criterion_main, Criterion, BenchmarkId };
use dod_color_bench::figure::{ Shape, Shapes, ShapeType, count_number_rect_oop, count_number_rect_dod };

fn generate_shape_list(number: u8) -> Vec<Shape> {
    let mut rng = rand::thread_rng();
    
    return (0..number).into_iter().map(|_| {
        return Shape {
            left_conner_x: rng.gen_range(1..100),
            left_conner_y: rng.gen_range(1..100),
            width: rng.gen_range(1..100),
            height: rng.gen_range(1..100),
            color: "#ff0000".to_string(),
            shape_type: random_shape_type(rng.gen_range(0..2))
        };
    }).collect::<Vec<Shape>>();
}

fn random_shape_type(number: u8) -> ShapeType {
    return match number {
        0 => ShapeType::RECT,
        _ => ShapeType::CIRCLE
    };
}

fn generate_shapes(number: u8) -> Shapes {
    let mut rng = rand::thread_rng();

    return (0..number).into_iter().fold(Shapes::new(), |mut acc, _| {
            acc.left_conners_x.push(rng.gen_range(1..100));
            acc.left_conners_y.push(rng.gen_range(1..100));
            acc.widths.push(rng.gen_range(1..100));
            acc.heights.push(rng.gen_range(1..100));
            acc.colors.push("#ff0000".to_string());
            acc.shapes_type.push(random_shape_type(rng.gen_range(0..2)));

            return acc;
        });
}

fn run_oop_dod_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("oop_dod");
    let bench_range: [u8; 6] = [10, 20, 50, 100, 150, 200];
    
    for number_element  in bench_range.iter() {
        group.bench_with_input(
        BenchmarkId::new("oop", number_element), 
        &generate_shape_list(*number_element), 
        |b, number_element| b.iter(|| count_number_rect_oop(number_element))
        );
        group.bench_with_input(
        BenchmarkId::new("dod", number_element),
        &generate_shapes(*number_element).shapes_type, 
        |b, number_element| b.iter(|| count_number_rect_dod(number_element))
        );
    }

    group.finish();
}

criterion_group!(benches, run_oop_dod_bench);
criterion_main!(benches);
