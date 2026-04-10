import { useState } from 'react';
import { Link } from 'chen-the-dawnstreak';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';
import Typography from '@mui/material/Typography';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import InputAdornment from '@mui/material/InputAdornment';
import Chip from '@mui/material/Chip';
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import SearchIcon from '@mui/icons-material/Search';
import AddIcon from '@mui/icons-material/Add';
import StarBorderIcon from '@mui/icons-material/StarBorder';
import ForkRightIcon from '@mui/icons-material/ForkRight';
import LockIcon from '@mui/icons-material/Lock';
import PublicIcon from '@mui/icons-material/Public';

interface Repo {
  name: string;
  owner: string;
  desc: string;
  lang: string;
  langColor: string;
  stars: number;
  forks: number;
  isPrivate: boolean;
  updated: string;
  topics: string[];
}

const repos: Repo[] = [
  {
    name: 'logos',
    owner: 'zixiao-labs',
    desc: '桌面代码编辑器，基于 Electron + Vue 3 + Monaco Editor，支持 SSH 远程开发和实时协作',
    lang: 'TypeScript',
    langColor: '#3178c6',
    stars: 342,
    forks: 28,
    isPrivate: false,
    updated: '2小时前',
    topics: ['editor', 'electron', 'vue3', 'collaboration'],
  },
  {
    name: 'chen-the-dawnstreak',
    owner: 'zixiao-labs',
    desc: '轻量级 React 元框架：文件路由、数据钩子、SSR、PWA、多平台 CLI 脚手架',
    lang: 'TypeScript',
    langColor: '#3178c6',
    stars: 156,
    forks: 12,
    isPrivate: false,
    updated: '5小时前',
    topics: ['react', 'framework', 'ssr'],
  },
  {
    name: 'aefanyl',
    owner: 'zixiao-labs',
    desc: '跨编辑器协作协议桥接，实现 Zed Protobuf RPC 到 WebSocket + JSON-RPC 的转换',
    lang: 'Rust',
    langColor: '#dea584',
    stars: 89,
    forks: 5,
    isPrivate: false,
    updated: '1天前',
    topics: ['collaboration', 'protocol', 'bridge'],
  },
  {
    name: 'nasti',
    owner: 'zixiao-labs',
    desc: '基于 Rust 的高性能前端打包工具',
    lang: 'Rust',
    langColor: '#dea584',
    stars: 67,
    forks: 3,
    isPrivate: false,
    updated: '2天前',
    topics: ['bundler', 'rust', 'toolchain'],
  },
  {
    name: 'devops-infra',
    owner: 'zixiao-labs',
    desc: '内部基础设施配置：Kubernetes 清单、Terraform 模块、CI/CD 模板',
    lang: 'HCL',
    langColor: '#844fba',
    stars: 0,
    forks: 0,
    isPrivate: true,
    updated: '3天前',
    topics: ['infrastructure', 'kubernetes'],
  },
  {
    name: 'yuxu',
    owner: 'zixiao-labs',
    desc: 'DevOps Service 平台（玉虚宫）：Git 仓库托管、议题、合并请求、CI/CD、实时协作',
    lang: 'TypeScript',
    langColor: '#3178c6',
    stars: 0,
    forks: 0,
    isPrivate: true,
    updated: '刚刚',
    topics: ['devops', 'platform', 'crdt'],
  },
];

export default function Repositories() {
  const [search, setSearch] = useState('');
  const [tab, setTab] = useState(0);

  const filtered = repos.filter((r) => {
    const matchSearch = r.name.toLowerCase().includes(search.toLowerCase()) ||
      r.desc.toLowerCase().includes(search.toLowerCase());
    if (tab === 1) return matchSearch && !r.isPrivate;
    if (tab === 2) return matchSearch && r.isPrivate;
    return matchSearch;
  });

  return (
    <Box>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h5">仓库</Typography>
        <Button variant="contained" startIcon={<AddIcon />} size="small">
          新建仓库
        </Button>
      </Box>

      <Box sx={{ display: 'flex', gap: 2, mb: 2, flexWrap: 'wrap', alignItems: 'center' }}>
        <TextField
          size="small"
          placeholder="搜索仓库..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          sx={{ minWidth: 280 }}
          slotProps={{
            input: {
              startAdornment: (
                <InputAdornment position="start">
                  <SearchIcon fontSize="small" sx={{ color: 'text.secondary' }} />
                </InputAdornment>
              ),
            },
          }}
        />
        <Tabs
          value={tab}
          onChange={(_, v) => setTab(v)}
          sx={{
            minHeight: 36,
            '& .MuiTab-root': { minHeight: 36, py: 0, fontSize: 13 },
          }}
        >
          <Tab label={`全部 (${repos.length})`} />
          <Tab label="公开" />
          <Tab label="私有" />
        </Tabs>
      </Box>

      <Box sx={{ display: 'flex', flexDirection: 'column', gap: 0 }}>
        {filtered.map((repo) => (
          <Paper
            key={repo.name}
            component={Link}
            to={`/repos/${repo.owner}/${repo.name}`}
            sx={{
              p: 2.5,
              display: 'block',
              textDecoration: 'none',
              color: 'inherit',
              borderRadius: 0,
              borderBottom: 1,
              borderColor: 'divider',
              '&:first-of-type': { borderTopLeftRadius: 8, borderTopRightRadius: 8 },
              '&:last-of-type': { borderBottomLeftRadius: 8, borderBottomRightRadius: 8, borderBottom: 0 },
              '&:hover': { bgcolor: 'rgba(124, 77, 255, 0.04)' },
              transition: 'background-color 0.15s',
            }}
          >
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 0.75 }}>
              <Typography variant="subtitle1" sx={{ fontWeight: 600, color: 'primary.light' }}>
                {repo.owner}/{repo.name}
              </Typography>
              <Chip
                icon={repo.isPrivate ? <LockIcon sx={{ fontSize: '14px !important' }} /> : <PublicIcon sx={{ fontSize: '14px !important' }} />}
                label={repo.isPrivate ? '私有' : '公开'}
                size="small"
                variant="outlined"
                sx={{ height: 20, fontSize: 11, '& .MuiChip-icon': { ml: 0.5 } }}
              />
            </Box>

            <Typography variant="body2" color="text.secondary" sx={{ mb: 1.5 }}>
              {repo.desc}
            </Typography>

            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2, flexWrap: 'wrap' }}>
              <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
                <Box sx={{ width: 12, height: 12, borderRadius: '50%', bgcolor: repo.langColor }} />
                <Typography variant="caption" color="text.secondary">{repo.lang}</Typography>
              </Box>
              {repo.stars > 0 && (
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.3 }}>
                  <StarBorderIcon sx={{ fontSize: 15, color: 'text.secondary' }} />
                  <Typography variant="caption" color="text.secondary">{repo.stars}</Typography>
                </Box>
              )}
              {repo.forks > 0 && (
                <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.3 }}>
                  <ForkRightIcon sx={{ fontSize: 15, color: 'text.secondary' }} />
                  <Typography variant="caption" color="text.secondary">{repo.forks}</Typography>
                </Box>
              )}
              {repo.topics.map((t) => (
                <Chip key={t} label={t} size="small" variant="outlined" sx={{ height: 20, fontSize: 11 }} />
              ))}
              <Typography variant="caption" color="text.secondary" sx={{ ml: 'auto' }}>
                更新于 {repo.updated}
              </Typography>
            </Box>
          </Paper>
        ))}
      </Box>
    </Box>
  );
}
