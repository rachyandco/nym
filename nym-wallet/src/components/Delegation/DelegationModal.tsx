import React from 'react';
import { Box, Button, Modal, Typography, SxProps } from '@mui/material';
import { Link } from '@nymproject/react/link/Link';
import { Console } from 'src/utils/console';
import { modalStyle } from '../Modals/styles';
import { LoadingModal } from '../Modals/LoadingModal';

export type ActionType = 'delegate' | 'undelegate' | 'redeem' | 'redeem-all' | 'compound';

const actionToHeader = (action: ActionType): string => {
  // eslint-disable-next-line default-case
  switch (action) {
    case 'redeem':
      return 'Rewards redeemed successfully';
    case 'redeem-all':
      return 'All rewards redeemed successfully';
    case 'delegate':
      return 'Delegation complete';
    case 'undelegate':
      return 'Undelegation complete';
    case 'compound':
      return 'Rewards compounded successfully';
    default:
      throw new Error('Unknown type');
  }
};

export type DelegationModalProps = {
  status: 'loading' | 'success' | 'error';
  action: ActionType;
  message?: string;
  recipient?: string;
  balance?: string;
  balanceVested?: string;
  transactions?: {
    url: string;
    hash: string;
  }[];
};

export const DelegationModal: React.FC<
  DelegationModalProps & {
    open: boolean;
    onClose?: () => void;
    sx?: SxProps;
    backdropProps?: object;
  }
> = ({
  status,
  action,
  message,
  recipient,
  balance,
  balanceVested,
  transactions,
  open,
  onClose,
  children,
  sx,
  backdropProps,
}) => {
  if (status === 'loading') return <LoadingModal sx={sx} backdropProps={backdropProps} />;

  if (status === 'error') {
    return (
      <Modal open={open} onClose={onClose} BackdropProps={backdropProps}>
        <Box sx={{ ...modalStyle, ...sx }} textAlign="center">
          <Typography color={(theme) => theme.palette.error.main} mb={1}>
            Oh no! Something went wrong...
          </Typography>
          <Typography my={5} color="text.primary">
            {message}
          </Typography>
          {children}
          <Button variant="contained" onClick={onClose}>
            Close
          </Button>
        </Box>
      </Modal>
    );
  }

  transactions?.map((transaction) => Console.log('action', action, 'status', status, 'key', transaction.hash));
  return (
    <Modal open={open} onClose={onClose} BackdropProps={backdropProps}>
      <Box sx={{ ...modalStyle, ...sx }} textAlign="center">
        <Typography color={(theme) => theme.palette.success.main} mb={1}>
          {actionToHeader(action)}
        </Typography>
        <Typography mb={3} color="text.primary">
          {message}
        </Typography>

        {recipient && (
          <Typography mb={1} fontSize="small" color={(theme) => theme.palette.text.secondary}>
            Recipient: {recipient}
          </Typography>
        )}
        {balanceVested ? (
          <>
            <Typography mb={1} fontSize="small" color={(theme) => theme.palette.text.secondary}>
              Your current balance: {balance?.toUpperCase()}
            </Typography>
            <Typography mb={1} fontSize="small" color={(theme) => theme.palette.text.secondary}>
              ({balanceVested.toUpperCase()} is unlocked in your vesting account)
            </Typography>
          </>
        ) : (
          <Typography mb={1} fontSize="small" color={(theme) => theme.palette.text.secondary}>
            Your current balance: {balance?.toUpperCase()}
          </Typography>
        )}
        {transactions && (
          <Typography mb={1} fontSize="small" color={(theme) => theme.palette.text.secondary}>
            Check the transaction {transactions.length > 1 ? 'hashes' : 'hash'}:
            {transactions.map((transaction) => (
              <Link
                key={transaction.hash}
                href={transaction.url}
                target="_blank"
                sx={{ ml: 1 }}
                text={transaction.hash.slice(0, 6)}
              />
            ))}
          </Typography>
        )}
        {children}
        <Button variant="contained" sx={{ mt: 3 }} size="large" onClick={onClose}>
          Finish
        </Button>
      </Box>
    </Modal>
  );
};
