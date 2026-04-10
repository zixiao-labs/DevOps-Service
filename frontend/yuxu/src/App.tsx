import { ChenRouter, Routes, Route } from 'chen-the-dawnstreak';
import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import theme from './theme';
import MainLayout from './layouts/MainLayout';
import Dashboard from './pages/Dashboard';
import Repositories from './pages/Repositories';
import RepositoryDetail from './pages/RepositoryDetail';
import Issues from './pages/Issues';
import MergeRequests from './pages/MergeRequests';
import CIPipelines from './pages/CIPipelines';
import Members from './pages/Members';
import Settings from './pages/Settings';

function App() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <ChenRouter>
        <Routes>
          <Route element={<MainLayout />}>
            <Route path="/" element={<Dashboard />} />
            <Route path="/repos" element={<Repositories />} />
            <Route path="/repos/:owner/:name" element={<RepositoryDetail />} />
            <Route path="/issues" element={<Issues />} />
            <Route path="/merge-requests" element={<MergeRequests />} />
            <Route path="/ci" element={<CIPipelines />} />
            <Route path="/members" element={<Members />} />
            <Route path="/settings" element={<Settings />} />
          </Route>
        </Routes>
      </ChenRouter>
    </ThemeProvider>
  );
}

export default App;
