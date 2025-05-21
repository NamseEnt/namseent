use database::schema::*;

pub trait IsTeamMember {
    fn is_team_member(&self, team_id: u128) -> bool;
}

impl IsTeamMember for TeamDoc {
    fn is_team_member(&self, team_id: u128) -> bool {
        self.member_ids.contains(&team_id)
    }
}
