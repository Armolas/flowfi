"use client";
import React, { useState } from "react";
import { Stepper } from "../ui/Stepper";
import { Button } from "../ui/Button";
import { RecipientStep } from "./RecipientStep";
import { TokenStep } from "./TokenStep";
import { AmountStep } from "./AmountStep";
import { ScheduleStep } from "./ScheduleStep";

export interface StreamFormData {
  recipient: string;
  token: string;
  amount: string;
  duration: string;
  durationUnit: "seconds" | "minutes" | "hours" | "days" | "weeks" | "months";
}

interface StreamCreationWizardProps {
  onClose: () => void;
  onSubmit: (data: StreamFormData) => Promise<void>;
}

const STEPS = ["Recipient", "Token", "Amount", "Schedule"];

export const StreamCreationWizard: React.FC<StreamCreationWizardProps> = ({
  onClose,
  onSubmit,
}) => {
  const [currentStep, setCurrentStep] = useState(1);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [formData, setFormData] = useState<StreamFormData>({
    recipient: "",
    token: "",
    amount: "",
    duration: "",
    durationUnit: "days",
  });
  const [errors, setErrors] = useState<Partial<Record<keyof StreamFormData, string>>>({});

  const updateFormData = (updates: Partial<StreamFormData>) => {
    setFormData((prev) => ({ ...prev, ...updates }));
    // Clear errors for updated fields
    setErrors((prev) => {
      const newErrors = { ...prev };
      Object.keys(updates).forEach((key) => {
        delete newErrors[key as keyof StreamFormData];
      });
      return newErrors;
    });
  };

  const validateStep = (step: number): boolean => {
    const newErrors: Partial<Record<keyof StreamFormData, string>> = {};

    switch (step) {
      case 1: // Recipient
        if (!formData.recipient.trim()) {
          newErrors.recipient = "Recipient address is required";
        } else if (!/^G[A-Z0-9]{55}$/.test(formData.recipient.trim())) {
          newErrors.recipient = "Invalid Stellar public key format";
        }
        break;
      case 2: // Token
        if (!formData.token) {
          newErrors.token = "Please select a token";
        }
        break;
      case 3: // Amount
        if (!formData.amount.trim()) {
          newErrors.amount = "Amount is required";
        } else {
          const amount = parseFloat(formData.amount);
          if (isNaN(amount) || amount <= 0) {
            newErrors.amount = "Amount must be a positive number";
          }
        }
        break;
      case 4: // Schedule
        if (!formData.duration.trim()) {
          newErrors.duration = "Duration is required";
        } else {
          const duration = parseFloat(formData.duration);
          if (isNaN(duration) || duration <= 0) {
            newErrors.duration = "Duration must be a positive number";
          }
        }
        break;
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleNext = () => {
    if (validateStep(currentStep)) {
      if (currentStep < STEPS.length) {
        setCurrentStep(currentStep + 1);
      }
    }
  };

  const handleBack = () => {
    if (currentStep > 1) {
      setCurrentStep(currentStep - 1);
    }
  };

  const handleSubmit = async () => {
    if (validateStep(currentStep)) {
      setIsSubmitting(true);
      try {
        await onSubmit(formData);
      } catch (error) {
        console.error("Failed to create stream:", error);
        // Error handling can be added here (e.g., toast notification)
      } finally {
        setIsSubmitting(false);
      }
    }
  };

  const renderStepContent = () => {
    switch (currentStep) {
      case 1:
        return (
          <RecipientStep
            value={formData.recipient}
            onChange={(value) => updateFormData({ recipient: value })}
            error={errors.recipient}
          />
        );
      case 2:
        return (
          <TokenStep
            value={formData.token}
            onChange={(value) => updateFormData({ token: value })}
            error={errors.token}
          />
        );
      case 3:
        return (
          <AmountStep
            value={formData.amount}
            onChange={(value) => updateFormData({ amount: value })}
            error={errors.amount}
            token={formData.token}
          />
        );
      case 4:
        return (
          <ScheduleStep
            duration={formData.duration}
            durationUnit={formData.durationUnit}
            onDurationChange={(value) => updateFormData({ duration: value })}
            onUnitChange={(value) => updateFormData({ durationUnit: value })}
            error={errors.duration}
            amount={formData.amount}
            token={formData.token}
          />
        );
      default:
        return null;
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div className="glass-card relative z-10 w-full max-w-2xl mx-4 max-h-[90vh] overflow-y-auto rounded-2xl border border-glass-border p-8">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-2xl font-bold">Create Payment Stream</h2>
          <button
            type="button"
            onClick={onClose}
            className="text-slate-400 hover:text-foreground transition-colors"
            aria-label="Close"
          >
            <svg
              className="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>

        <Stepper steps={STEPS} currentStep={currentStep} />

        <div className="my-8 min-h-[300px]">{renderStepContent()}</div>

        <div className="flex justify-between gap-4 pt-6 border-t border-glass-border">
          <div>
            {currentStep > 1 && (
              <Button variant="outline" onClick={handleBack}>
                Back
              </Button>
            )}
          </div>
          <div className="flex gap-4">
            <Button variant="outline" onClick={onClose}>
              Cancel
            </Button>
            {currentStep < STEPS.length ? (
              <Button onClick={handleNext}>Next</Button>
            ) : (
              <Button onClick={handleSubmit} disabled={isSubmitting}>
                {isSubmitting ? "Creating..." : "Create Stream"}
              </Button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
