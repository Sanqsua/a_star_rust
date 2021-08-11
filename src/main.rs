//configure window
fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "A star algorithm".to_owned(),
        fullscreen: false,
        window_width: 1600,
        window_height: 1000,
        ..macroquad::window::Conf::default()
    }
}

#[derive(Clone,Debug)]
pub struct A_Star_Node {
    b_obstacle: bool,
    b_visited: bool,
    f_global_goal: f32,
    f_local_goal: f32,
    u_x_pos: u32,
    u_y_pos: u32,
    vec_neighbours: Vec<A_Star_Node>,
    struct_parent_node: Option<Box<A_Star_Node>>
}
impl A_Star_Node{
    pub fn new() -> Self {
        Self {
            b_obstacle: false,
            b_visited: false,
            f_global_goal: std::f32::MAX,
            f_local_goal: std::f32::MAX,
            u_x_pos: 0,
            u_y_pos: 0,
            vec_neighbours: Vec::new(),
            struct_parent_node: None,
        }
    }
}
#[macroquad::main(window_conf())]
async fn main() {
    const MAP_WIDTH: usize = 9;
    const MAP_HEIGHT: usize = 9;
    const MAP_DIMENSION: usize = MAP_WIDTH * MAP_HEIGHT;
    const NODE_SIZE: usize = 50;
    const NODE_BORDER: usize = 20;
    let mut nodes_v: Vec<A_Star_Node> = Vec::with_capacity(MAP_DIMENSION);
    //debug shit
    let mut mouse_clicked_text: String = String::from("mouse clicked at: ''");
    let mut node_clicked_pos_text: String = String::from("node clicked : ''");
    let mut mouse_position_text: String;
    let mut node_information_text: String = String::from("Node information");
    //fill the the nodes_vec
    for _ in 0..MAP_DIMENSION {
        nodes_v.push(A_Star_Node::new());
    }

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            nodes_v[y * MAP_WIDTH + x].u_x_pos = x as u32;
            nodes_v[y * MAP_WIDTH + x].u_y_pos = y as u32;
            nodes_v[y * MAP_WIDTH + x].b_obstacle = false;
            nodes_v[y * MAP_WIDTH + x].struct_parent_node = None;
            nodes_v[y * MAP_WIDTH + x].b_visited = false;
        }
    }

    
    //this here is needed so it doesn't recursivly have all nodes
    /* 
    let neighbour_nodes_v : Vec<A_Star_Node> = nodes_v.to_owned();
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            if y > 0 {
                let neighbour_north : A_Star_Node =  neighbour_nodes_v[(y-1)*MAP_WIDTH+(x+0)].to_owned();
                nodes_v[y*MAP_WIDTH+x].vec_neighbours.push(neighbour_north);
            }
            if x< MAP_WIDTH-1{
            let neighbour_east : A_Star_Node =neighbour_nodes_v[(y+0)*MAP_WIDTH+(x+1)].to_owned();
                nodes_v[y*MAP_WIDTH+x].vec_neighbours.push(neighbour_east);
            }
            if y < MAP_HEIGHT-1{
                let neighbour_south : A_Star_Node = neighbour_nodes_v[(y+1)*MAP_WIDTH+(x+0)].to_owned();
                nodes_v[y*MAP_WIDTH+x].vec_neighbours.push(neighbour_south);
            }
            if x>0 {
                let neighbour_west: A_Star_Node = neighbour_nodes_v[(y+0)*MAP_WIDTH+(x-1)].to_owned();
                nodes_v[y*MAP_WIDTH+x].vec_neighbours.push(neighbour_west);
            }
        }
    }
    */
   
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            if y > 0 {
                let neighbour_north : A_Star_Node =  nodes_v[(y-1)*MAP_WIDTH+(x+0)].to_owned();
                nodes_v[y*MAP_WIDTH+x].vec_neighbours.push(neighbour_north);
            }
            if x< MAP_WIDTH-1{
            let neighbour_east : A_Star_Node =nodes_v[(y+0)*MAP_WIDTH+(x+1)].to_owned();
                nodes_v[y*MAP_WIDTH+x].vec_neighbours.push(neighbour_east);
            }
            if y < MAP_HEIGHT-1{
                let neighbour_south : A_Star_Node = nodes_v[(y+1)*MAP_WIDTH+(x+0)].to_owned();
                nodes_v[y*MAP_WIDTH+x].vec_neighbours.push(neighbour_south);
            }
            if x>0 {
                let neighbour_west: A_Star_Node = nodes_v[(y+0)*MAP_WIDTH+(x-1)].to_owned();
                nodes_v[y*MAP_WIDTH+x].vec_neighbours.push(neighbour_west);
            }
        }
    }
   
    

    /*
    if node.local < neighbour.local +dstance with na
    give new node parent
    update local
    update global
    */

    /*
    macroquad::shapes::draw_rectangle(x: f32, y: f32, w: f32, h: f32, color: Color);
    macroquad::shapes::draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color);
    */
    loop {
        macroquad::window::clear_background(macroquad::color::WHITE);
        //keyinput
        if macroquad::input::is_key_released(macroquad::input::KeyCode::Escape) {
            break;
        }

        if macroquad::input::is_mouse_button_released(macroquad::input::MouseButton::Left) {
            let total_mouse_position_x: usize = macroquad::input::mouse_position().0 as usize;
            let total_mouse_position_y: usize = macroquad::input::mouse_position().1 as usize;
            //boundaries of the grid or the application will crash
            if total_mouse_position_x < MAP_WIDTH * NODE_SIZE && total_mouse_position_y < MAP_HEIGHT * NODE_SIZE
            {
                let selected_node_pos_x: usize = macroquad::input::mouse_position().0 as usize / NODE_SIZE;
                let selected_node_pos_y: usize = macroquad::input::mouse_position().1 as usize / NODE_SIZE;
            //boundaries of the rectangles (only the blue and yellow ones) 
                if total_mouse_position_x >= selected_node_pos_x*NODE_SIZE+NODE_BORDER/2 && total_mouse_position_y >= selected_node_pos_y*NODE_SIZE+NODE_BORDER/2
                && total_mouse_position_x <= selected_node_pos_x*NODE_SIZE+(NODE_SIZE-NODE_BORDER/2) && total_mouse_position_y <= selected_node_pos_y*NODE_SIZE+(NODE_SIZE-NODE_BORDER/2){
                    node_clicked_pos_text = format!("node clicked {}{}", selected_node_pos_x,selected_node_pos_y);
                    nodes_v[selected_node_pos_y * MAP_WIDTH + selected_node_pos_x].b_obstacle = !nodes_v[selected_node_pos_y * MAP_WIDTH + selected_node_pos_x].b_obstacle;
                    node_information_text = format!("{:?}",nodes_v[selected_node_pos_y* MAP_WIDTH+selected_node_pos_y].vec_neighbours);
                    println!("{:?}",node_information_text) ;
                }
            }
        }
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                
                // the rectangles are overlapping. so the "w" and "h" of the draw_rectangle function need subtract the full border to have an even shit
                macroquad::shapes::draw_rectangle((x*NODE_SIZE) as f32, (y*NODE_SIZE)as f32, NODE_SIZE as f32, NODE_SIZE as  f32, macroquad::color::BLACK);
                macroquad::shapes::draw_rectangle( (x * NODE_SIZE + NODE_BORDER/2) as f32, (y * NODE_SIZE + NODE_BORDER/2) as f32,
                    (NODE_SIZE - NODE_BORDER) as f32, (NODE_SIZE - NODE_BORDER ) as f32, 
                    match nodes_v[y * MAP_WIDTH + x].b_obstacle {
                        true => macroquad::color::YELLOW,
                        false => macroquad::color::BLUE,
                    },
                );
               // macroquad::shapes::draw_line(0f32 , (y*NODE_SIZE) as f32,  (MAP_WIDTH*NODE_SIZE) as f32, (y*NODE_SIZE) as f32, 10f32, macroquad::color::GREEN);
            }
                //macroquad::shapes::draw_line((x*NODE_SIZE) as f32 , 0 as f32, (x*NODE_SIZE) as f32,  (MAP_WIDTH*NODE_SIZE) as f32, 10f32, macroquad::color::PURPLE);
        }

        //debug
        if macroquad::input::is_mouse_button_released(macroquad::input::MouseButton::Left) {
            mouse_clicked_text =
                format!("mouse clicked at {:?}", macroquad::input::mouse_position());
        }

        mouse_position_text = format!("mouse position {:?}", macroquad::input::mouse_position());

        macroquad::text::draw_text( mouse_position_text.as_ref(), macroquad::window::screen_width() / 2f32, (1 * NODE_SIZE + NODE_BORDER) as f32, 50f32, macroquad::color::RED,);
        macroquad::text::draw_text( mouse_clicked_text.as_ref(), macroquad::window::screen_width() / 2f32, (2 * NODE_SIZE + NODE_BORDER) as f32, 50f32, macroquad::color::RED,);
        macroquad::text::draw_text( node_clicked_pos_text.as_ref(), macroquad::window::screen_width() / 2f32, (3 * NODE_SIZE + NODE_BORDER) as f32, 50f32, macroquad::color::RED,);
        macroquad::text::draw_text( "Press ESC to End", macroquad::window::screen_width() / 2f32, (4 * NODE_SIZE + NODE_BORDER) as f32, 50f32, macroquad::color::RED,);
        macroquad::text::draw_text( node_information_text.as_ref(),  0f32, (20 * NODE_SIZE + NODE_BORDER) as f32, 50f32, macroquad::color::RED,);

        macroquad::window::next_frame().await
    }
}
