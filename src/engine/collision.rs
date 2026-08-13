use crate::renderer::world::World;

pub fn intersect(world: &World, a: usize, b: usize) -> bool {
    let obj_a = &world.models[a]; 
    let obj_b = &world.models[b]; 
    let a_min = obj_a.position - (obj_a.scale);
    let a_max = obj_a.position + (obj_a.scale);
    let b_min = obj_b.position - (obj_b.scale);
    let b_max = obj_b.position + (obj_b.scale);

    return 
        a_min.x <= b_max.x && 
        a_max.x >= b_min.x && 
        a_min.y <= b_max.y && 
        a_max.y >= b_min.y && 
        a_min.z <= b_max.z && 
        a_max.z >= b_min.z 
    
}
