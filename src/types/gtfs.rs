mod gtfs {
    pub struct Agency {
        id: i32,
        name: String,
        url: String,
        timezone: String,
        language: String,
        phone: String
    }

    pub struct Calendar {

    }

    pub struct CalendarDate {

    }

    pub struct Transfer {

    }

    pub struct Stop {

    }

    pub struct Route {
        id: i32,
        agency: Agency,
        short_name: String,
        long_name: String,
        color: String,
        text_color: String,
    }

    pub struct Trip {
        route: Route,
    }


}
