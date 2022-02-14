use crate::task::set_current_priority;

pub fn sys_set_priority(prio: isize) -> isize {
    set_current_priority(prio)
}
