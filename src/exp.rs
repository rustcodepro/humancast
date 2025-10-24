use anyhow::{Context, Result};
use ndarray::{Array1, Array2, Axis};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

/*
Gaurav Sablok
codeprog@icloud.com
*/

fn read_csv<P: AsRef<Path>>(path: P) -> Result<Array2<f64>> {
    let file = File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);
    let mut gene_ids = Vec::new();
    let mut counts = Vec::new();

    for result in rdr.records() {
        let record = result?;
        gene_ids.push(record[0].to_string());
        let row: Vec<f64> = record
            .iter()
            .skip(1)
            .map(|s| s.parse().unwrap_or(0.0))
            .collect();
        counts.push(row);
    }
    let n_genes = gene_ids.len();
    let n_samples = if n_genes > 0 { counts[0].len() } else { 0 };
    let mut data = Array2::zeros((n_genes, n_samples));
    for (i, row) in counts.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            data[[i, j]] = val;
        }
    }
    Ok(data)
}

fn compute_library_sizes(data: &Array2<f64>) -> Array1<f64> {
    let (_, n_samples) = data.dim();
    let mut lib_sizes = Array1::zeros(n_samples);
    for j in 0..n_samples {
        lib_sizes[j] = data.column(j).sum();
    }
    lib_sizes
}

fn compute_tmm_factors(
    data: &Array2<f64>,
    lib_sizes: &Array1<f64>,
    ref_sample: usize,
) -> Result<Array1<f64>> {
    let (n_genes, n_samples) = data.dim();
    let mut factors = Array1::zeros(n_samples);
    let ref_lib_size = lib_sizes[ref_sample];

    for j in 0..n_samples {
        if j == ref_sample {
            factors[j] = 1.0;
            continue;
        }
        let mut m_values: Vec<f64> = Vec::new();
        let mut a_values: Vec<f64> = Vec::new();
        let mut total_weight = 0.0;

        for i in 0..n_genes {
            let count_i_j = data[[i, j]];
            let count_i_ref = data[[i, ref_sample]];
            if count_i_j == 0.0 || count_i_ref == 0.0 {
                continue;
            }

            let log_count_i_j = (count_i_j / lib_sizes[j]).ln();
            let log_count_i_ref = (count_i_ref / ref_lib_size).ln();
            let m = log_count_i_j - log_count_i_ref;
            let a = 0.5 * (log_count_i_j + log_count_i_ref);
            let denom = count_i_j + count_i_ref;
            let weight = 1.0 / (1.0 + denom * 0.0001);
            m_values.push(m);
            a_values.push(a);
            total_weight += weight;
        }
        if m_values.is_empty() {
            factors[j] = 1.0;
            continue;
        }
        let mut indices: Vec<usize> = (0..m_values.len()).collect();
        indices.sort_by(|&x, &y| a_values[x].partial_cmp(&a_values[y]).unwrap());
        let trim = (0.3 * m_values.len() as f64).round() as usize;
        let start = trim;
        let end = m_values.len() - trim;
        let mut trimmed_m_sum = 0.0;
        let mut trimmed_weight_sum = 0.0;

        for k in start..end {
            let idx = indices[k];
            let w = 1.0 / (1.0 + (count_i_ref + data[[indices[k], j]]) * 0.0001);
            trimmed_m_sum += m_values[idx] * w;
            trimmed_weight_sum += w;
        }

        let trimmed_mean_m = if trimmed_weight_sum > 0.0 {
            trimmed_m_sum / trimmed_weight_sum
        } else {
            0.0
        };
        factors[j] = (-trimmed_mean_m).exp();
    }

    Ok(factors)
}

fn normalize_to_logcpm(
    data: &Array2<f64>,
    lib_sizes: &Array1<f64>,
    tmm_factors: &Array1<f64>,
) -> Result<Array2<f64>> {
    let (n_genes, n_samples) = data.dim();
    let mut normalized = Array2::zeros((n_genes, n_samples));
    let log10_offset = 6.0; // log10(1e6) for CPM

    for j in 0..n_samples {
        let effective_lib_size = lib_sizes[j] * tmm_factors[j] * 1e-6;
        for i in 0..n_genes {
            let count = data[[i, j]];
            let cpm = if effective_lib_size > 0.0 {
                count / effective_lib_size
            } else {
                0.0
            };
            normalized[[i, j]] = cpm.log2().max(0.0);
        }
    }

    Ok(normalized)
}

fn write_output<P: AsRef<Path>>(normalized: &Array2<f64>, path: P) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = io::BufWriter::new(file);
    let (n_genes, n_samples) = normalized.dim();
    for j in 0..n_samples {
        if j > 0 {
            write!(writer, "\t")?;
        }
        write!(writer, "Sample{}", j + 1)?;
    }
    writeln!(writer)?;
    for i in 0..n_genes {
        for j in 0..n_samples {
            if j > 0 {
                write!(writer, "\t")?;
            }
            write!(writer, "{:.2}", normalized[[i, j]])?;
        }
        writeln!(writer)?;
    }
    Ok(())
}
