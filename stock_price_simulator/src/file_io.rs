use crate::simulation::SimulationInputs;

use csv::Writer;
use std::error::Error;

pub fn write_simulation_results(
    file_path: &str,
    simulation_results: SimulationInputs,
    n_steps: usize,
    n_paths: usize,
) -> Result<(), Box<dyn Error>> {
    println!("Writing file to {}", file_path);
    let mut writer = Writer::from_path(file_path)?;
    writer.write_record(&["path_index", "time_index", "price"])?;

    for i in 0..n_steps {
        for j in 0..n_paths {
            let _ = writer.write_record(&[
                j.to_string(),
                simulation_results.time_index[i].to_string(),
                simulation_results.paths[[i, j]].to_string(),
            ]);
        }
    }
    Ok(())
}
