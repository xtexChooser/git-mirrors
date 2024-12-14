// Copyright 2024 The Forgejo Authors. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later

package forgejo_migrations //nolint:revive

import "xorm.io/xorm"

func AddSubIssuesRelationship(x *xorm.Engine) error {
	type Issue struct {
		ID            int64  `xorm:"pk autoincr"`
		ParentIssueID *int64 `xorm:"'parent_id' null index"`
	}

	return x.Sync(&Issue{})
}
