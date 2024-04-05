-- Additional table for the GlobalUserrights extension
-- To be added to $wgSharedDB

CREATE TABLE /*_*/global_user_groups (
  -- Key to user_id
  gug_user serial NOT NULL,
  -- Group name
  gug_group varchar(16) NOT NULL default '',

  PRIMARY KEY (gug_user, gug_group)
) /*$wgDBTableOptions*/;

CREATE INDEX /*i*/gug_group ON /*_*/global_user_groups (gug_group);
