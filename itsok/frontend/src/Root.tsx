import { AppBar, Box, Container, Toolbar } from "@mui/material";
import { Link, Outlet } from "@tanstack/react-router";

export function Root() {
  return (
    <>
      <AppBar>
        <Toolbar>
          <Link to="/">Home</Link>
        </Toolbar>
      </AppBar>
      <Toolbar />
      {/* ^- https://github.com/mui/material-ui/issues/16844#issuecomment-517205129 */}
      <Container maxWidth="md">
        <Box m={4}>
          <Outlet />
        </Box>
      </Container>
    </>
  );
}
