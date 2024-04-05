-- Changes the length of groups from 14 to 255 similar to user_group
ALTER TABLE /*_*/global_user_groups ALTER gug_group TYPE varchar(255);
ALTER TABLE /*_*/global_user_groups ALTER gug_group SET NOT NULL;
ALTER TABLE /*_*/global_user_groups ALTER gug_group SET DEFAULT '';
