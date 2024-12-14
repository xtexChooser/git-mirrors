// Copyright 2024 The Forgejo Authors. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later

package issues_test

import (
	"fmt"
	"testing"

	"code.gitea.io/gitea/models/db"
	issues_model "code.gitea.io/gitea/models/issues"
	"code.gitea.io/gitea/models/unittest"
	user_model "code.gitea.io/gitea/models/user"
	"code.gitea.io/gitea/modules/setting"

	"github.com/stretchr/testify/require"
)

func TestIssueUpdateParent(t *testing.T) {
	require.NoError(t, unittest.PrepareTestDatabase())

	user1 := unittest.AssertExistsAndLoadBean(t, &user_model.User{ID: 1})
	parentIssue := unittest.AssertExistsAndLoadBean(t, &issues_model.Issue{ID: 23})
	subIssue := unittest.AssertExistsAndLoadBean(t, &issues_model.Issue{ID: 24})

	err := subIssue.UpdateParentIssue(db.DefaultContext, parentIssue, user1)
	require.NoError(t, err)

	unittest.AssertExistsAndLoadBean(t, &issues_model.Issue{ID: 23, ParentIssueID: nil})
	unittest.AssertExistsAndLoadBean(t, &issues_model.Issue{ID: 24, ParentIssueID: &parentIssue.ID})
	unittest.AssertExistsAndLoadBean(t, &issues_model.Comment{
		Type:               issues_model.CommentTypeAddSubIssue,
		PosterID:           user1.ID,
		IssueID:            parentIssue.ID,
		ParentOrSubIssueID: subIssue.ID,
	})
	unittest.AssertExistsAndLoadBean(t, &issues_model.Comment{
		Type:               issues_model.CommentTypeAddParentIssue,
		PosterID:           user1.ID,
		IssueID:            subIssue.ID,
		ParentOrSubIssueID: parentIssue.ID,
	})

	err = subIssue.UpdateParentIssue(db.DefaultContext, nil, user1)
	require.NoError(t, err)

	unittest.AssertExistsAndLoadBean(t, &issues_model.Issue{ID: 23, ParentIssueID: nil})
	unittest.AssertExistsAndLoadBean(t, &issues_model.Issue{ID: 24, ParentIssueID: nil})
	unittest.AssertExistsAndLoadBean(t, &issues_model.Comment{
		Type:               issues_model.CommentTypeRemoveSubIssue,
		PosterID:           user1.ID,
		IssueID:            parentIssue.ID,
		ParentOrSubIssueID: subIssue.ID,
	})
	unittest.AssertExistsAndLoadBean(t, &issues_model.Comment{
		Type:               issues_model.CommentTypeRemoveParentIssue,
		PosterID:           user1.ID,
		IssueID:            subIssue.ID,
		ParentOrSubIssueID: parentIssue.ID,
	})
}

func TestSubIssueCountLimit(t *testing.T) {
	require.NoError(t, unittest.PrepareTestDatabase())

	user1 := unittest.AssertExistsAndLoadBean(t, &user_model.User{ID: 1})
	parentIssue := unittest.AssertExistsAndLoadBean(t, &issues_model.Issue{ID: 23})
	for i := 0; i < setting.Repository.Issue.MaxSubIssues; i++ {
		issue := testCreateIssue(t, 63, 1, fmt.Sprintf("Test sub-issue No. %d", i), "issue content", false)
		err := issue.UpdateParentIssue(db.DefaultContext, parentIssue, user1)
		require.NoError(t, err)
	}
	issue := testCreateIssue(t, 63, 1, "last test issue", "issue content", false)
	err := issue.UpdateParentIssue(db.DefaultContext, parentIssue, user1)
	require.EqualError(t, err, issues_model.ErrSubIssuesTooMany{issue.ID, parentIssue.ID, parentIssue.ID}.Error())
}

func TestSubIssueDepthLimit(t *testing.T) {
	require.NoError(t, unittest.PrepareTestDatabase())

	user1 := unittest.AssertExistsAndLoadBean(t, &user_model.User{ID: 1})
	rootIssue := unittest.AssertExistsAndLoadBean(t, &issues_model.Issue{ID: 23})
	issue := rootIssue
	for i := 0; i < setting.Repository.Issue.MaxSubIssuesDepth; i++ {
		newIssue := testCreateIssue(t, 63, 1, fmt.Sprintf("Test sub-issue No. %d", i), "issue content", false)
		err := newIssue.UpdateParentIssue(db.DefaultContext, issue, user1)
		require.NoError(t, err)
		issue = newIssue
	}
	newIssue := testCreateIssue(t, 63, 1, "last test issue", "issue content", false)
	err := newIssue.UpdateParentIssue(db.DefaultContext, issue, user1)
	require.EqualError(t, err, issues_model.ErrSubIssuesTooDeep{newIssue.ID, issue.ID, rootIssue.ID}.Error())
}

func TestSubIssueNoCircular(t *testing.T) {
	require.NoError(t, unittest.PrepareTestDatabase())

	user1 := unittest.AssertExistsAndLoadBean(t, &user_model.User{ID: 1})
	rootIssue := unittest.AssertExistsAndLoadBean(t, &issues_model.Issue{ID: 23})

	issue1 := testCreateIssue(t, 63, 1, "Test sub-issue", "issue content", false)
	err := issue1.UpdateParentIssue(db.DefaultContext, rootIssue, user1)
	require.NoError(t, err)

	err = rootIssue.UpdateParentIssue(db.DefaultContext, issue1, user1)
	require.EqualError(t, err, issues_model.ErrCircularParentIssue{rootIssue.ID, issue1.ID}.Error())
}
