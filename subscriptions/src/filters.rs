use gamedig::valve_master_server::{Filter, SearchFilters};
use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject, Debug)]
pub struct ServersFilters {
    is_secured: Option<bool>,
    runs_map: Option<String>,
    can_have_password: Option<bool>,
    can_be_empty: Option<bool>,
    is_empty: Option<bool>,
    can_be_full: Option<bool>,
    runs_app_id: Option<i32>,
    not_app_id: Option<i32>,
    has_tags: Option<Vec<String>>,
    match_name: Option<String>,
    match_version: Option<String>,
    restrict_unique_ip: Option<bool>,
    on_address: Option<String>,
    whitelisted: Option<bool>,
    spectator_proxy: Option<bool>,
    is_dedicated: Option<bool>,
    runs_linux: Option<bool>,
    has_game_dir: Option<String>
}

impl ServersFilters {
    pub fn to_gamedig_filters(self) -> Vec<Filter> {
        let mut filters = Vec::new();

        if let Some(is_secured) = self.is_secured {
            filters.push(Filter::IsSecured(is_secured))
        }
        if let Some(runs_map) = self.runs_map {
            filters.push(Filter::RunsMap(runs_map))
        }
        if let Some(can_have_password) = self.can_have_password {
            filters.push(Filter::CanHavePassword(can_have_password))
        }
        if let Some(can_be_empty) = self.can_be_empty {
            filters.push(Filter::CanBeEmpty(can_be_empty))
        }
        if let Some(is_empty) = self.is_empty {
            filters.push(Filter::IsEmpty(is_empty))
        }
        if let Some(can_be_full) = self.can_be_full {
            filters.push(Filter::CanBeFull(can_be_full))
        }
        if let Some(runs_app_id) = self.runs_app_id {
            filters.push(Filter::RunsAppID(runs_app_id as u32))
        }
        if let Some(not_app_id) = self.not_app_id {
            filters.push(Filter::NotAppID(not_app_id as u32))
        }
        if let Some(has_tags) = self.has_tags {
            filters.push(Filter::HasTags(has_tags))
        }
        if let Some(match_name) = self.match_name {
            filters.push(Filter::MatchName(match_name))
        }
        if let Some(match_version) = self.match_version {
            filters.push(Filter::MatchVersion(match_version))
        }
        if let Some(restrict_unique_ip) = self.restrict_unique_ip {
            filters.push(Filter::RestrictUniqueIP(restrict_unique_ip))
        }
        if let Some(on_address) = self.on_address {
            filters.push(Filter::OnAddress(on_address))
        }
        if let Some(whitelisted) = self.whitelisted {
            filters.push(Filter::Whitelisted(whitelisted))
        }
        if let Some(spectator_proxy) = self.spectator_proxy {
            filters.push(Filter::SpectatorProxy(spectator_proxy))
        }
        if let Some(is_dedicated) = self.is_dedicated {
            filters.push(Filter::IsDedicated(is_dedicated))
        }
        if let Some(runs_linux) = self.runs_linux {
            filters.push(Filter::RunsLinux(runs_linux))
        }
        if let Some(has_game_dir) = self.has_game_dir {
            filters.push(Filter::HasGameDir(has_game_dir))
        }

        filters
    }
}

pub fn to_gamedig_filters(filters: Option<ServersFilters>, nor_filters: Option<ServersFilters>, nand_filters: Option<ServersFilters>) -> SearchFilters {
    let mut search_filters = SearchFilters::new();

    if let Some(the_filters) = filters {
        for filter in the_filters.to_gamedig_filters() {
            search_filters = search_filters.insert(filter);
        }
    }

    if let Some(the_filters) = nor_filters {
        for filter in the_filters.to_gamedig_filters() {
            search_filters = search_filters.insert_nor(filter);
        }
    }

    if let Some(the_filters) = nand_filters {
        for filter in the_filters.to_gamedig_filters() {
            search_filters = search_filters.insert_nand(filter);
        }
    }

    search_filters
}
