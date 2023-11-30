pub enum InstanceParams {
    Server(Vec<String>, i32),
    Client(Option<String>, String, i32)
}


/**
 Reads the options entered at startup.<br>
 The following structures are allowed:<br>
 Server creation: -s \<port> -r \<roomname1>,\<roomname2>,...,\<roomnamen> <br>
 Client creation: -c \<address>:\<port> -u \<username> <br>
 */
pub fn parse_arguments(mut args: Vec<String>) -> Option<InstanceParams> {
    args.remove(0);

    // Returns Some(port) if the "-s" option is set and has a value (-> server creation)
    if let Some(port) = extract_server_port(&mut args) {
        let rooms = extract_chatroom_names(&mut args)?;
        
        // Makes sure that there are no other arguments left (as not to mix server creation and client connection in the same command)
        return if args.len() == 0 { Some(InstanceParams::Server(rooms, port)) }else { None };
    } 
    
    // Returns Some((address, port)) if the "-c" option is set and has a value (-> client connection)
    if let Some((address, port)) = extract_connect_info(&mut args) {
        let username = extract_username(&mut args);
        
        return if args.len() == 0 { Some(InstanceParams::Client(username, address, port)) } else { None };
    }

    return None;
}

fn extract_server_port(args: &mut Vec<String>) -> Option<i32> {
    return extract_value_after(args, "-s")?.parse::<i32>().ok();   
}

fn extract_connect_info(args: &mut Vec<String>) -> Option<(String, i32)> {
    let connect_info_string = extract_value_after(args, "-c")?;
    let connect_info_parts = connect_info_string.split(":").collect::<Vec<&str>>();
    
    if connect_info_parts.len() != 2 {
        return None;
    } 

    let port = connect_info_parts[1].parse::<i32>().ok()?;
    return Some((connect_info_parts[0].to_string(), port));
}

fn extract_username(args: &mut Vec<String>) -> Option<String> {
    return extract_value_after(args, "-u");
}

fn extract_chatroom_names(args: &mut Vec<String>) -> Option<Vec<String>> {
    let chatroom_names_string = extract_value_after(args, "-r")?;
    return Some(chatroom_names_string.split(",").map(|e| e.to_string()).collect::<Vec<String>>());
}

fn extract_value_after(args: &mut Vec<String>, opt: &str) -> Option<String> {
    let opt_index = args.iter().position(|s| s.as_str() == opt)?;
    let val = args.get(opt_index + 1).cloned()?;
    if opt_index + 1 < args.len() {
        args.remove(opt_index);
        args.remove(opt_index);
    }
    return Some(val);
}