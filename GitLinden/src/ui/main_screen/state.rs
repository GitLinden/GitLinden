use springtime_di::factory::ComponentFactoryBuilder;

use crate::ui::branch_list::BranchList;

pub(crate) struct MainScreen {
    pub(crate) branch_list: BranchList,
}

impl MainScreen {
    pub(crate) fn new() -> MainScreen {
        let mut factory = ComponentFactoryBuilder::new()
            .expect("Error initializing ComponentFactoryBuilder")
            .build();

        MainScreen {
            branch_list: BranchList::new(&mut factory),
        }
    }
}
