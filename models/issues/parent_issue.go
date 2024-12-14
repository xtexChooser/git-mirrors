// Copyright 2024 The Forgejo Authors. All rights reserved.
// SPDX-License-Identifier: GPL-3.0-or-later

package issues

import (
	"context"
	"fmt"

	"code.gitea.io/gitea/models/db"
	user_model "code.gitea.io/gitea/models/user"
	"code.gitea.io/gitea/modules/setting"
	"code.gitea.io/gitea/modules/util"
)

// LoadParentIssue load parent issue of this issue.
func (issue *Issue) LoadParentIssue(ctx context.Context) (err error) {
	if issue.ParentIssueID != nil && !issue.isParentIssueLoaded && (issue.ParentIssue == nil || issue.ParentIssue.ID != *issue.ParentIssueID) {
		issue.ParentIssue, err = GetIssueByID(ctx, *issue.ParentIssueID)
		if err != nil {
			return err
		}

		issue.isParentIssueLoaded = true
	}

	return nil
}

// GetSubIssuesByIssueID returns all sub-issues that belong to given issue by ID.
func GetSubIssuesByIssueID(ctx context.Context, issueID int64) ([]*Issue, error) {
	var subIssues []*Issue
	return subIssues, db.GetEngine(ctx).
		Table("issue").
		Where("issue.parent_id = ?", issueID).
		Asc("issue.created_unix").
		Find(&subIssues)
}

// LoadSubIssues load sub-issues of this issue.
func (issue *Issue) LoadSubIssues(ctx context.Context) (err error) {
	if !issue.isSubIssuesLoaded {
		issue.SubIssues, err = GetSubIssuesByIssueID(ctx, issue.ID)
		if err != nil {
			return err
		}
		issue.isSubIssuesLoaded = true
	}
	return nil
}

// ErrSubIssuesTooMany represents a "SubIssuesTooMany" kind of error.
type ErrSubIssuesTooMany struct {
	ID       int64
	ParentID int64
	RootID   int64
}

// IsErrSubIssuesTooMany checks if an error is a ErrSubIssuesTooMany.
func IsErrSubIssuesTooMany(err error) bool {
	_, ok := err.(ErrSubIssuesTooMany)
	return ok
}

func (err ErrSubIssuesTooMany) Error() string {
	return fmt.Sprintf("sub-issues count has reached limit [id: %d, parent: %d, root: %d]", err.ID, err.ParentID, err.RootID)
}

func (err ErrSubIssuesTooMany) Unwrap() error {
	return util.ErrNotExist
}

// ErrSubIssuesTooDeep represents a "SubIssuesTooDeep" kind of error.
type ErrSubIssuesTooDeep struct {
	ID       int64
	ParentID int64
	RootID   int64
}

// IsErrSubIssuesTooDeep checks if an error is a ErrSubIssuesTooDeep.
func IsErrSubIssuesTooDeep(err error) bool {
	_, ok := err.(ErrSubIssuesTooDeep)
	return ok
}

func (err ErrSubIssuesTooDeep) Error() string {
	return fmt.Sprintf("sub-issues depth has reached limit [id: %d, parent: %d, root: %d]", err.ID, err.ParentID, err.RootID)
}

// ErrCircularParentIssue represents a "CircularParentIssue" kind of error.
type ErrCircularParentIssue struct {
	ID       int64
	ParentID int64
}

// IsErrCircularParentIssue checks if an error is a ErrCircularParentIssue.
func IsErrCircularParentIssue(err error) bool {
	_, ok := err.(ErrCircularParentIssue)
	return ok
}

func (err ErrCircularParentIssue) Error() string {
	return fmt.Sprintf("circular parent issues [id: %d, parent: %d]", err.ID, err.ParentID)
}

// LookupRootIssue resolves the root issue of this issue, which has no parent issue
func (issue *Issue) LookupRootIssue(ctx context.Context) (root *Issue, depth int, err error) {
	root = issue
	depth = 0
	for {
		if root.ParentIssueID == nil {
			return root, depth, nil
		}
		if err = root.LoadParentIssue(ctx); err != nil {
			return nil, 0, err
		}
		root = root.ParentIssue
		depth++
	}
}

// CountSubIssues counts count of all sub-issues of this issue recursively
func (issue *Issue) CountSubIssues(ctx context.Context) (count int, err error) {
	if err = issue.LoadSubIssues(ctx); err != nil {
		return 0, err
	}
	count = len(issue.SubIssues)
	for _, subissue := range issue.SubIssues {
		subcount, err := subissue.CountSubIssues(ctx)
		if err != nil {
			return 0, err
		}
		count += subcount
	}
	return count, nil
}

// UpdateParentIssue adds issue to another issue as a sub-issue.
// Setting parent issue to nil means removing parent issue
func (issue *Issue) UpdateParentIssue(ctx context.Context, parent *Issue, doer *user_model.User) (err error) {
	if issue.ParentIssueID != nil && parent != nil && *issue.ParentIssueID == parent.ID {
		return nil
	}
	if issue.ParentIssueID == nil && parent == nil {
		return nil
	}
	if err = issue.LoadParentIssue(ctx); err != nil {
		return err
	}
	oldParent := issue.ParentIssue

	if parent != nil {
		root, depth, err := parent.LookupRootIssue(ctx)
		if err != nil {
			return err
		}

		// Validate count and depth limitation
		if depth+1 > setting.Repository.Issue.MaxSubIssuesDepth {
			return ErrSubIssuesTooDeep{issue.ID, parent.ID, root.ID}
		}
		rootCount, err := root.CountSubIssues(ctx)
		if err != nil {
			return err
		}
		if rootCount+1 > setting.Repository.Issue.MaxSubIssues {
			return ErrSubIssuesTooMany{issue.ID, parent.ID, root.ID}
		}

		// Validate no circular parent issues
		curParent := parent
		// after parent.LookupRootIssue, all parent issues on the path to root has been loaded
		for curParent != nil {
			if curParent.ID == issue.ID {
				return ErrCircularParentIssue{issue.ID, parent.ID}
			}
			curParent = curParent.ParentIssue
		}
	}

	ctx, committer, err := db.TxContext(ctx)
	if err != nil {
		return err
	}
	defer committer.Close()

	var parentID *int64
	if parent != nil {
		parentID = &parent.ID
	} else {
		parentID = nil
	}

	// Update parent ID
	if err = UpdateIssueCols(ctx, &Issue{ID: issue.ID, ParentIssueID: parentID}, "parent_id"); err != nil {
		return err
	}

	issue.ParentIssueID = parentID
	// invalidates caches
	issue.isParentIssueLoaded = false
	if parent != nil {
		parent.isSubIssuesLoaded = false
	}

	// Make the comment
	if oldParent != nil {
		// removed old parent
		if err = createSubIssueComment(ctx, doer, oldParent, issue, false); err != nil {
			return fmt.Errorf("createSubIssueComment, unlink old: %w", err)
		}
	}
	if parent != nil {
		// added new parent
		if err = createSubIssueComment(ctx, doer, parent, issue, true); err != nil {
			return fmt.Errorf("createSubIssueComment, link new: %w", err)
		}
	}

	return committer.Commit()
}
