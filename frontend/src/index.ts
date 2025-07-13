import { uiManager } from './ui';
import './styles.css';

// Initialize the application when DOM is loaded
document.addEventListener('DOMContentLoaded', async () => {
  try {
    await uiManager.initializeApp();
  } catch (error) {
    console.error('Failed to initialize application:', error);
  }
});