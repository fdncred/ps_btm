use bottom::app::{self, data_harvester, DataFilters};

fn main() {
    // let disk_filter =
    //     get_ignore_list(&config.disk_filter).context("Update 'disk_filter' in your config file")?;
    // let mount_filter = get_ignore_list(&config.mount_filter)
    //     .context("Update 'mount_filter' in your config file")?;
    // let temp_filter =
    //     get_ignore_list(&config.temp_filter).context("Update 'temp_filter' in your config file")?;
    // let net_filter =
    //     get_ignore_list(&config.net_filter).context("Update 'net_filter' in your config file")?;

    let disk_filter = None;
    let mount_filter = None;
    let temp_filter = None;
    let net_filter = None;

    let filters = DataFilters {
        disk_filter,
        mount_filter,
        temp_filter,
        net_filter,
    };

    // .filters(DataFilters {
    //     disk_filter,
    //     mount_filter,
    //     temp_filter,
    //     net_filter,
    // })

    // let temp_type = app_config_fields.temperature_type.clone();
    // let use_current_cpu_total = app_config_fields.use_current_cpu_total;
    // let show_average_cpu = app_config_fields.show_average_cpu;

    // let used_widgets = UsedWidgets {
    //     use_cpu: used_widget_set.get(&Cpu).is_some() || used_widget_set.get(&BasicCpu).is_some(),
    //     use_mem: used_widget_set.get(&Mem).is_some() || used_widget_set.get(&BasicMem).is_some(),
    //     use_net: used_widget_set.get(&Net).is_some() || used_widget_set.get(&BasicNet).is_some(),
    //     use_proc: used_widget_set.get(&Proc).is_some(),
    //     use_disk: used_widget_set.get(&Disk).is_some(),
    //     use_temp: used_widget_set.get(&Temp).is_some(),
    //     use_battery: used_widget_set.get(&Battery).is_some(),
    // };

    let used_widgets = app::layout_manager::UsedWidgets {
        use_cpu: true,
        use_mem: true,
        use_net: true,
        use_proc: true,
        use_disk: true,
        use_temp: true,
        use_battery: true,
    };
    let temp_type = data_harvester::temperature::TemperatureType::Fahrenheit;
    let use_current_cpu_total = true;
    let show_average_cpu = false;

    let mut data_state = data_harvester::DataCollector::new(filters);
    data_state.set_collected_data(used_widgets);
    data_state.set_temperature_type(temp_type);
    data_state.set_use_current_cpu_total(use_current_cpu_total);
    data_state.set_show_average_cpu(show_average_cpu);

    data_state.init();
    futures::executor::block_on(data_state.update_data());

    // println!("{:#?}", data_state);
    // println!("{:#?/}", data_state.data.list_of_processes.unwrap());

    println!("pid|parent_pid|cpu_use_percent|mem_use_percent|mem_use_bytes|name|command|read_bytes_per_sec|write_bytes_per_sec|total_read_bytes|total_write_bytes|process_state|process_state_char|uid");
    match data_state.data.list_of_processes {
        Some(procs) => {
            for x in procs {
                let pid = x.pid;
                let parent_pid = match x.parent_pid {
                    Some(p) => p,
                    None => 0,
                };
                let cpu_use_percent = x.cpu_usage_percent;
                let mem_use_percent = x.mem_usage_percent;
                let mem_use_bytes = x.mem_usage_bytes;
                let name = x.name;
                let command = x.command;
                let read_bytes_per_sec = x.read_bytes_per_sec;
                let write_bytes_per_sec = x.write_bytes_per_sec;
                let total_read_bytes = x.total_read_bytes;
                let total_write_bytes = x.total_write_bytes;
                let process_state = x.process_state;
                let process_state_char = x.process_state_char;
                let uid = match x.uid {
                    Some(p) => p,
                    None => 0,
                };
                println!(
                    "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
                    pid,
                    parent_pid,
                    cpu_use_percent,
                    mem_use_percent,
                    mem_use_bytes,
                    name,
                    command,
                    read_bytes_per_sec,
                    write_bytes_per_sec,
                    total_read_bytes,
                    total_write_bytes,
                    process_state,
                    process_state_char,
                    uid,
                )
            }
        }
        None => {}
    }
}
