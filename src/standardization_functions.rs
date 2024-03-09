//low price 24h
	pub fn xlm_lognorm_standardization_low_price_24h(input: &f64) -> f64 {
		let mean = -2.208930172;
		let std_dev = 1.032873359;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn xrp_lognorm_standardization_low_price_24h(input: &f64) -> f64 {
		let mean = -0.862589085;
		let std_dev = 0.545833231;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn sol_lognorm_standardization_low_price_24h(input: &f64) -> f64 {
		let mean = 3.760933;
		let std_dev = 0.8262162;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}


//lot volume per trade
	pub fn xlm_lognorm_standardization_lot_volume_per_trade(input: &f64) -> f64 {
		let mean = 5.794098781;
		let std_dev = 2.34694218;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn xrp_lognorm_standardization_lot_volume_per_trade(input: &f64) -> f64 {
		let mean = 6.877477435;
		let std_dev = 2.465247989;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn sol_lognorm_standardization_lot_volume_per_trade(input: &f64) -> f64 {
		let mean = 7.365294875;
		let std_dev = 2.685845322;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}


//high price 24h
	pub fn xlm_lognorm_standardization_high_price_24h(input: &f64) -> f64 {
		let mean = -2.208164848;
		let std_dev = 1.032876105;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn xrp_lognorm_standardization_high_price_24h(input: &f64) -> f64 {
		let mean = -0.861608649;
		let std_dev = 0.546126923;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn sol_lognorm_standardization_high_price_24h(input: &f64) -> f64 {
		let mean = 3.761668948;
		let std_dev = 0.82638822;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}


//close price 24h
	pub fn xlm_lognorm_standardization_close_price_24h(input: &f64) -> f64 {
		let mean = -2.208476998;
		let std_dev = 1.032885018;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn xrp_lognorm_standardization_close_price_24h(input: &f64) -> f64 {
		let mean = -0.862061284;
		let std_dev = 0.546001909;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn sol_lognorm_standardization_close_price_24h(input: &f64) -> f64 {
		let mean = 3.761254764;
		let std_dev = 0.826299771;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}


//total trades 24h
	pub fn xlm_lognorm_standardization_total_trades_24h(input: &f64) -> f64 {
		let mean = 7.099004493;
		let std_dev = 1.222749449;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn xrp_lognorm_standardization_total_trades_24h(input: &f64) -> f64 {
		let mean = 8.322607727;
		let std_dev = 0.808927553;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn sol_lognorm_standardization_total_trades_24h(input: &f64) -> f64 {
		let mean = 9.028825496;
		let std_dev = 0.852114562;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}


//total volume 24h
	pub fn xlm_lognorm_standardization_total_volume_24h(input: &f64) -> f64 {
		let mean = 15.31962609;
		let std_dev = 1.044065831;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn xrp_lognorm_standardization_total_volume_24h(input: &f64) -> f64 {
		let mean = 16.10628557;
		let std_dev = 0.875821451;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn sol_lognorm_standardization_total_volume_24h(input: &f64) -> f64 {
		let mean = 12.48386429;
		let std_dev = 0.85293782;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}


//vwap 24h
	pub fn xlm_lognorm_standardization_vwap_24h(input: &f64) -> f64 {
		let mean = -2.208890131;
		let std_dev = 1.03243165;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn xrp_lognorm_standardization_vwap_24h(input: &f64) -> f64 {
		let mean = -0.862436281;
		let std_dev = 0.545295204;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn sol_lognorm_standardization_vwap_24h(input: &f64) -> f64 {
		let mean = 3.761964168;
		let std_dev = 0.826939756;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}


//open price 24h
	pub fn xlm_lognorm_standardization_open_price_24h(input: &f64) -> f64 {
		let mean = -2.20861243;
		let std_dev = 1.03288034;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn xrp_lognorm_standardization_open_price_24h(input: &f64) -> f64 {
		let mean = -0.862116684;
		let std_dev = 0.545998465;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}

	pub fn sol_lognorm_standardization_open_price_24h(input: &f64) -> f64 {
		let mean = 3.761329463;
		let std_dev = 0.826305098;
		let log_input = input.ln();
		(log_input - mean) / std_dev
	}


//price % change 24h
	pub fn xlm_normal_standardization_price_percent_change_24h(input: &f64) -> f64 {
		let mean = 1.14144708090948;
		let std_dev = 43.12892747;
		(input - mean) / std_dev
	}

	pub fn xrp_normal_standardization_price_percent_change_24h(input: &f64) -> f64 {
		let mean = 0.229526313414368;
		let std_dev = 6.671949238;
		(input - mean) / std_dev
	}

	pub fn sol_normal_standardization_price_percent_change_24h(input: &f64) -> f64 {
		let mean = 0.308075719666597;
		let std_dev = 6.241447845;
		(input - mean) / std_dev
	}


//high 52w
	pub fn xlm_normal_standardization_high_52w(input: &f64) -> f64 {
		let mean = 0.407952667;
		let std_dev = 0.228029234;
		(input - mean) / std_dev
	}

	pub fn xrp_normal_standardization_high_52w(input: &f64) -> f64 {
		let mean = 1.085263333;
		let std_dev = 0.554081686;
		(input - mean) / std_dev
	}

	pub fn sol_normal_standardization_high_52w(input: &f64) -> f64 {
		let mean = 87.255;
		let std_dev = 38.875;
		(input - mean) / std_dev
	}


//low 52w
	pub fn xlm_normal_standardization_low_52w(input: &f64) -> f64 {
		let mean = 0.074584167;
		let std_dev = 0.045073906;
		(input - mean) / std_dev
	}

	pub fn xrp_normal_standardization_low_52w(input: &f64) -> f64 {
		let mean = 0.281805;
		let std_dev = 0.123369028;
		(input - mean) / std_dev
	}

	pub fn sol_normal_standardization_low_52w(input: &f64) -> f64 {
		let mean = 11.055;
		let std_dev = 3.105;
		(input - mean) / std_dev
	}


//standardize wallets
	pub fn normal_wallet_standardization(input: &f64) -> f64 {
		let mean = 500.568712;
		let std_dev = 500.0;
		(input - mean) / std_dev
	}

	pub fn normal_value_prior_standardization(input: &f64) -> f64 {
		let mean = 2000.568712;
		let std_dev = 2000.0;
		(input - mean) / std_dev
	}