import {expect} from '@playwright/test';
import {test} from './utils_e2e.ts';

test('Contributor graph', async ({page}) => {
  await page.goto('/user2/commits_search_test/activity/contributors');
  await page.getByRole('link', {name: '2 Commits'}).click();
  await expect(page.getByRole('cell', {name: 'Bob'})).toHaveCount(2);
});
