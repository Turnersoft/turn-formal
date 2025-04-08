interface ContentItem {
  id: string;
  title: string;
  content: string;
  category: string;
  path: string;
}

interface ContentCollection {
  items: ContentItem[];
}

/**
 * Fetch a list of available content categories
 */
export const fetchContentCategories = async (): Promise<string[]> => {
  try {
    const response = await fetch("/content/index.json");
    if (!response.ok) {
      throw new Error("Failed to fetch content categories");
    }
    const data = await response.json();
    return data.categories || [];
  } catch (error) {
    console.error("Error fetching content categories:", error);
    return [];
  }
};

/**
 * Fetch content items for a specific category
 */
export const fetchCategoryContent = async (
  category: string
): Promise<ContentItem[]> => {
  try {
    const response = await fetch(`/content/${category}.json`);
    if (!response.ok) {
      throw new Error(`Failed to fetch content for category: ${category}`);
    }
    const data: ContentCollection = await response.json();
    return data.items || [];
  } catch (error) {
    console.error(`Error fetching content for category ${category}:`, error);
    return [];
  }
};

/**
 * Find a specific content item by ID
 */
export const fetchContentItemById = async (
  category: string,
  itemId: string
): Promise<ContentItem | null> => {
  try {
    const items = await fetchCategoryContent(category);
    return items.find((item) => item.id === itemId) || null;
  } catch (error) {
    console.error(`Error fetching content item ${itemId}:`, error);
    return null;
  }
};

/**
 * Search across all content categories
 */
export const searchContent = async (query: string): Promise<ContentItem[]> => {
  try {
    const categories = await fetchContentCategories();
    const allContentPromises = categories.map(fetchCategoryContent);
    const allCategoriesContent = await Promise.all(allContentPromises);

    // Flatten array of arrays and filter by search query
    const allContent = allCategoriesContent.flat();
    const normalizedQuery = query.toLowerCase();

    return allContent.filter(
      (item) =>
        item.title.toLowerCase().includes(normalizedQuery) ||
        item.content.toLowerCase().includes(normalizedQuery)
    );
  } catch (error) {
    console.error("Error searching content:", error);
    return [];
  }
};
