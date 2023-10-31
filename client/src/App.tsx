import React from 'react';
import './App.css';
import { Layout, Alert, Spin, Switch, Card,Table } from 'antd';

const {useState, useEffect } = React;

function App() {
  const [todos, setTodos] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    setLoading(true);
    // fetch for type application/json
    fetch(`/api/todos`)
      .then((response) => {
        console.log(response);
        setLoading(false);
        if (!response.ok) {
          throw new Error(
            `This is an HTTP error: The status is ${response.status}`
          );
        }
        return response.json();
      })
      .then((actualData) => {
        setError(null);
        setTodos(actualData.todos);
        console.log(actualData)
      })
      .catch((err) => {
        setError(err.message);
        setTodos([]);
        console.log(err.message);
      });
  }, []);

  let columns =  [ 
    {
      title: 'Title',
      dataIndex: 'title',
      key: 'title',
    }, 
  ];
  return (
    <Layout>
      <Layout.Header>
        Header
      </Layout.Header>
      <Layout.Content>
        <Card title="TODOs">
          <Spin spinning={loading}>
            {error && <Alert message={error} type="error" />}
            {loading && <Alert message="Loading TODOs..." description="Requesting data from the server..." type="info" />}
          </Spin>
          <Table columns={columns} dataSource={todos}>
          </Table>
        </Card>
      </Layout.Content>
      <Layout.Footer>
        Built with an LLM.
      </Layout.Footer>
    </Layout>
  );
}


export default App;
