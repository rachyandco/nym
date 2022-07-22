import React, { useState } from 'react';
import { Box } from '@mui/material';
import { CurrencyDenom, TNodeType } from '@nymproject/types';
import { SimpleModal } from 'src/components/Modals/SimpleModal';
import { MixnodeForm } from '../forms/MixnodeForm';
import { AmountData, GatewayData, MixnodeData } from 'src/pages/bonding/types';
import { GatewayForm } from '../forms/GatewayForm';

const defaultGatewayalues: GatewayData = {
  identityKey: '2UB4668XV7qhmJDPp6KLGWGisiaUYThjA4in2o7WKcwA',
  sphinxKey: '5Rh7X4TwMoUwrQ1ivkqWTCGi1pivmHtenaS7VZDUQPYW',
  ownerSignature: '3ccrgwiHhqAbuhhdW7f6UCHZoPFJsQxPcSQRwNc42QVDnDwW8Ebe8p51RhvQp28uqpARysPz52XrE6JuuwJ6fsf8',
  host: '1.1.1.1',
  version: '1.1.1',
  location: '',
  mixPort: 1789,
  clientsPort: 9000,
  advancedOpt: false,
};

const defaultAmountValues = (denom: CurrencyDenom): AmountData => ({
  amount: { amount: '100', denom },
  profitMargin: 10,
  tokenPool: 'balance',
});

export const BondGatewayModal = ({
  denom,
  hasVestingTokens,
  onClose,
}: {
  denom: CurrencyDenom;
  hasVestingTokens: boolean;
  onClose: () => void;
}) => {
  const [step, setStep] = useState<1 | 2>(1);
  const [gatewayData, setGatewayData] = useState<GatewayData>(defaultGatewayalues);
  const [amountData, setAmountData] = useState<AmountData>(defaultAmountValues(denom));

  const handleBack = () => {
    validateStep(2);
    setStep(1);
  };

  const validateStep = (step: number) => {
    const event = new CustomEvent('validate_gateway_step', { detail: { step } });
    window.dispatchEvent(event);
  };

  const handleUpdateGatewayData = (data: GatewayData) => {
    setGatewayData(data);
    setStep(2);
  };

  const handleUpdateAmountData = (data: AmountData) => {
    setAmountData(data);
  };

  const handleSubmit = async () => {};

  return (
    <SimpleModal
      open
      onOk={async () => validateStep(step)}
      onBack={step === 2 ? handleBack : undefined}
      onClose={onClose}
      header="Bond"
      subHeader={`Step ${step}/2`}
      okLabel="Next"
    >
      <Box sx={{ mb: 2 }}>
        <GatewayForm
          step={step}
          hasVestingTokens={hasVestingTokens}
          denom={denom}
          onValidateGatewayData={handleUpdateGatewayData}
          onValidateAmountData={handleUpdateAmountData}
          gatewayData={gatewayData}
          amountData={amountData}
        />
      </Box>
    </SimpleModal>
  );
};
