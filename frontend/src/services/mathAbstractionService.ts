import { mockAbstractionData } from '../mocks/abstractionData';

/**
 * Service for fetching mathematical abstraction data from the backend
 */
export async function fetchGroupTheoryAbstractionData(): Promise<Record<string, any>> {
  try {
    // In a production environment, this would be a real API call
    // For this demo, we'll use mock data
    
    // Simulate API call delay
    await new Promise(resolve => setTimeout(resolve, 500));
    
    // In production, this would be:
    // const response = await fetch('/api/math/theories/groups/abstraction_data');
    // const data = await response.json();
    // return data;
    
    // For now, return mock data
    return mockAbstractionData;
  } catch (error) {
    console.error('Error fetching group theory abstraction data:', error);
    // Fallback to mock data
    return mockAbstractionData;
  }
} 