use crate::*;

#[document]
struct TeamDoc {
    #[pk]
    id: String,
    name: String,
}

#[document]
struct UserToTeamDoc {
    #[pk]
    user_id: String,
    #[sk]
    team_id: String,
}

#[document]
struct TeamNameToTeamIdDoc {
    #[pk]
    team_name: String,
    team_id: String,
}

// pub struct TeamNameToTeamIdDocUpdate<WantUpdateFn, UpdateFn>
// where
//     WantUpdateFn: Send + FnOnce(&ArchivedTeamNameToTeamIdDoc) -> WantUpdate,
//     UpdateFn: Send + FnOnce(&mut TeamNameToTeamIdDoc),
// {
//     pub want_update: WantUpdateFn,
//     pub update: UpdateFn,
// }

// impl<'a, WantUpdateFn, UpdateFn> TryInto<document::TransactItem<'a>>
//     for TeamNameToTeamIdDocUpdate<WantUpdateFn, UpdateFn>
// where
//     WantUpdateFn: Send + FnOnce(&ArchivedTeamNameToTeamIdDoc) -> WantUpdate,
//     UpdateFn: Send + FnOnce(&mut TeamNameToTeamIdDoc),
// {
//     type Error = document::SerErr;
//     fn try_into(self) -> document::Result<document::TransactItem<'a>> {
//         Ok(document::TransactItem::Update {
//             name: todo!(),
//             pk: todo!(),
//             sk: todo!(),
//             update_fn: Some(Box::new(|vec| {
//                 let want_update =
//                     (self.want_update)(unsafe { rkyv::archived_root::<TeamNameToTeamIdDoc>(vec) });

//                 if let WantUpdate::Yes = want_update {
//                     let mut doc = deserialize::<TeamNameToTeamIdDoc>(vec)?;
//                     (self.update)(&mut doc);
//                     *vec = serialize(&doc)?;
//                 }

//                 Ok(want_update)
//             })),
//             // name: stringify!(#name),
//             // pk: #pk_cow,
//             // sk: #sk_cow,
//             // value: #ref_struct_value,
//             // ttl: self.ttl,
//         })
//     }
// }
