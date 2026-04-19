import { useNavigate } from 'chen-the-dawnstreak';
import { Button, Card } from '@heroui/react';

export default function NotFound() {
  const navigate = useNavigate();
  return (
    <div
      className="flex min-h-screen items-center justify-center p-6"
      style={{ background: 'var(--background)' }}
    >
      <Card className="w-full max-w-md">
        <Card.Header>
          <Card.Title>页面不存在</Card.Title>
          <Card.Description>你访问的页面已下线或从未存在。</Card.Description>
        </Card.Header>
        <Card.Footer>
          <Button onPress={() => navigate('/')}>返回首页</Button>
        </Card.Footer>
      </Card>
    </div>
  );
}
