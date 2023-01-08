use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert from CityGML to a graphics representation like glTF
    ConvertToGraphics {
        /// Path to the CityGML dataset
        #[clap(short, long)]
        input_path: String,

        /// Path to the output directory
        #[clap(short, long)]
        output_path: Option<String>,

        #[clap(long, number_of_values = 3, allow_hyphen_values = true)]
        corner_min: Option<Vec<f64>>,

        #[clap(long, number_of_values = 3, allow_hyphen_values = true)]
        corner_max: Option<Vec<f64>>,

        /// offset which is subtracted from the geocoordinates
        #[clap(long, number_of_values = 3, allow_hyphen_values = true)]
        offset: Option<Vec<f64>>,
    },

    /// Convert from CityGML to a ROS2 bag for visualization purposes
    ConvertToRosbag {
        /// Path to the CityGML dataset
        #[clap(short, long)]
        input_path: String,

        /// Path to the output directory
        #[clap(short, long)]
        rosbag_directory_path: String,

        #[clap(long, number_of_values = 3, allow_hyphen_values = true)]
        corner_min: Option<Vec<f64>>,

        #[clap(long, number_of_values = 3, allow_hyphen_values = true)]
        corner_max: Option<Vec<f64>>,

        /// offset which is subtracted from the geocoordinates
        #[clap(long, number_of_values = 3, allow_hyphen_values = true)]
        offset: Option<Vec<f64>>,
    },
}
