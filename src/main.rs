const WIDTH: usize = 30;
const HEIGHT: usize = 30;
const LAYOUT: &str = include_str!("./layout.txt");

#[derive(Debug, Clone, PartialEq)]
enum CellKind {
    Wall,
    Air,
    Goal,
}
#[derive(Debug, Clone)]
struct Cell {
    possibilities: Vec<CellKind>,
    x: usize,
    y: usize
}
impl Cell {
    fn new(x: usize, y: usize) -> Self {
        Self {
            possibilities: vec![CellKind::Wall, CellKind::Air, CellKind::Goal],
            x,
            y
        }
    }
    fn from_possibility_vec(possibilities: Vec<CellKind>, x: usize, y: usize) -> Self {
        Self {
            possibilities,
            x,
            y
        }
    }
    fn collapsed(&self) -> Option<CellKind> {
        if self.possibilities.len() == 1 {
            return Some(self.possibilities[0].clone());
        }
        None
    }
}


fn is_in_grid(x: usize, y: usize) -> bool {
    if x < WIDTH && x >= 0 {
        if y < HEIGHT && y >= 0 {
            return true;
        }
    }
    false
}
fn set_grid(grid: &mut Vec<Cell>, x: usize, y: usize, value: Cell) {
    if is_in_grid(x, y) {
        grid[(x * HEIGHT + y)] = value;
    }
}
fn get_grid(grid: &Vec<Cell>, x: usize, y: usize) -> Option<Cell> {
    if is_in_grid(x, y) {
        return Some(grid[(x * HEIGHT + y)].clone());
    }
    None
}
fn can_have_around(kind: CellKind) -> Vec<CellKind> {
    match kind {
        CellKind::Wall => vec![CellKind::Wall, CellKind::Air],
        CellKind::Air => vec![CellKind::Wall, CellKind::Air, CellKind::Goal],
        CellKind::Goal => vec![CellKind::Air],
    }
}
fn find_lowest_possibility_count(grid: &Vec<Cell>) -> usize {
    let mut lowest = 100;
    for cell in grid.iter() {
        let count = cell.possibilities.len();
        if count < lowest && count != 1 {
            lowest = count;
        }
    }
    lowest
}
fn intersection<T: PartialEq + Clone>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut output = Vec::new();
    for compare in a.iter() {
        if b.contains(compare) {
            output.push(compare.clone());
        }
    }
    output
}
fn update(grid: &mut Vec<Cell>, x: usize, y: usize) {
    let possibilities = vec![pick_random::<CellKind>(&get_grid(grid, x, y).unwrap().possibilities.clone())];
    println!("{:?}", possibilities);
    set_grid(grid, x, y, Cell::from_possibility_vec(possibilities, x, y));
    let neighboring_possibility = can_have_around(get_grid(&grid, x, y).unwrap().collapsed().unwrap());
    // top
    if y > 1 {
    if let Some(cell) = get_grid(grid, x, y-1) {
        let possibility = Cell::from_possibility_vec(
             intersection::<CellKind>(
                    &cell.possibilities,
                    &neighboring_possibility
                ), x, y-1
             );
        set_grid(grid, x, y-1, possibility);
    }
    }
    // right
    if let Some(cell) = get_grid(grid, x+1, y) {
        let possibility = Cell::from_possibility_vec(
             intersection::<CellKind>(
                    &cell.possibilities,
                    &neighboring_possibility
                ), x+1, y
             );
        set_grid(grid, x+1, y, possibility);
    }
    // bottom
    if let Some(cell) = get_grid(grid, x, y+1) {
        let possibility = Cell::from_possibility_vec(
             intersection::<CellKind>(
                    &cell.possibilities,
                    &neighboring_possibility
                ), x, y+1
             );
        set_grid(grid, x, y+1, possibility);
    }
    // left
    if x > 1 {
    if let Some(cell) = get_grid(grid, x-1, y) {
        let possibility = Cell::from_possibility_vec(
             intersection::<CellKind>(
                    &cell.possibilities,
                    &neighboring_possibility
                ), x-1, y
             );
        set_grid(grid, x-1, y, possibility);
    }
    }
}
fn pick_random<T: Clone>(vector: &Vec<T>) -> T {
    let pick_index = rand::random::<usize>() % vector.len();
    vector[pick_index].clone()
}

fn main() {
    let mut grid: Vec<Cell> = Vec::new();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            grid.push(Cell::new(x, y));
        }
    }
    let random_x = rand::random::<usize>() % WIDTH;
    let random_y = rand::random::<usize>() % HEIGHT;
    set_grid(&mut grid, random_x, random_y, Cell::from_possibility_vec(vec![CellKind::Wall], random_x, random_y));

    
    for i in 0..600 {
        let lowest_entropy = find_lowest_possibility_count(&grid);
        let selection: Vec<Cell> = grid.iter().filter(|x| x.possibilities.len() == lowest_entropy).map(|x| x.clone()).collect();
        if selection.len() == 0 {
            panic!("lol");
        }
        let pick = pick_random::<Cell>(&selection);
        update(&mut grid, pick.x, pick.y);
    }

    draw(&grid);
}

fn draw(grid: &Vec<Cell>) {
    // draw the grid
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let index = (x * HEIGHT + y) as usize;
            if let Some(kind) = grid[index].collapsed() {
                let c = match kind {
                    CellKind::Wall => {'#'},
                    CellKind::Air => {' '},
                    CellKind::Goal => {'g'}
                };
                print!("{}{}", c, c);
            } else {
                print!("..");
            }
            //print!("{}", get_grid(grid, x, y).possibilities.len());
        }
        println!();
    }
}
