use pyo3::prelude::*;
use pyo3::types::PyDict;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendationInput {
    pub user_id: Uuid,
    pub user_preferences: Vec<String>,
    pub reading_history: Vec<Uuid>, // Book IDs
    pub ratings: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendationOutput {
    pub book_ids: Vec<Uuid>,
    pub scores: Vec<f64>,
    pub confidence: f64,
}

pub struct MLModelService {
    model_path: String,
}

impl MLModelService {
    pub fn new(model_path: String) -> Self {
        Self { model_path }
    }

    pub fn get_recommendations(&self, input: RecommendationInput) -> Result<RecommendationOutput> {
        Python::with_gil(|py| {
            // Load the Python module
            let sys = py.import("sys")?;
            let path: &PyAny = sys.getattr("path")?;
            path.call_method1("append", ("./python_models",))?;

            // Import required modules
            let pickle = py.import("pickle")?;
            let numpy = py.import("numpy")?;

            // Load the trained model
            let model_file = std::fs::File::open(&self.model_path)?;
            let model: &PyAny = pickle.call_method1("load", (model_file,))?;

            // Prepare input data
            let user_features = self.prepare_user_features(&input, py, numpy)?;
            
            // Get predictions
            let predictions: &PyAny = model.call_method1("predict", (user_features,))?;
            let scores: &PyAny = model.call_method1("predict_proba", (user_features,))?;

            // Extract results
            let book_ids: Vec<Uuid> = predictions.extract()?;
            let confidence_scores: Vec<f64> = scores.extract()?;
            
            let avg_confidence = confidence_scores.iter().sum::<f64>() / confidence_scores.len() as f64;

            Ok(RecommendationOutput {
                book_ids,
                scores: confidence_scores,
                confidence: avg_confidence,
            })
        })
    }

    fn prepare_user_features(
        &self,
        input: &RecommendationInput,
        py: Python,
        numpy: &PyAny,
    ) -> PyResult<&PyAny> {
        // Convert user preferences to feature vector
        let preferences_dict = PyDict::new(py);
        for pref in &input.user_preferences {
            preferences_dict.set_item(pref, 1.0)?;
        }

        // Convert reading history to features
        let history_array = numpy.call_method1("array", (input.reading_history.clone(),))?;
        let ratings_array = numpy.call_method1("array", (input.ratings.clone(),))?;

        // Combine features (this would depend on your specific model requirements)
        let features = numpy.call_method1("concatenate", ((preferences_dict, history_array, ratings_array),))?;
        
        Ok(features)
    }
}