import { useContext } from 'react';
import { AppContext } from 'src/context/main';
import { BondingContextProvider, useBondingContext } from '../../context';
import { PageLayout } from '../../layouts';
import BondingCard from './bonding';
import GatewayCard from './gateway';
import MixnodeCard from './mixnode';

const Bonding = () => {
  const { bondedMixnode, bondedGateway } = useBondingContext();

  // TODO display a special UI on loading state
  return (
    <PageLayout>
      {!bondedMixnode && !bondedGateway && <BondingCard />}
      {bondedMixnode && <MixnodeCard mixnode={bondedMixnode} />}
      {bondedGateway && <GatewayCard gateway={bondedGateway} />}
    </PageLayout>
  );
};

export const BondingPage = () => {
  const { network } = useContext(AppContext);
  return (
    <BondingContextProvider network={network}>
      <Bonding />
    </BondingContextProvider>
  );
};
